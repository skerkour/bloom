use chrono::offset::TimeZone;
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use error::{Error, ErrorKind};
use nom::{types::CompleteStr as Input, *};
use std::collections::BTreeSet;
use std::collections::Bound::{Included, Unbounded};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::iter::{self, Iterator};
use std::str::{self, FromStr};

use time_unit::*;

#[derive(Clone, Debug)]
pub struct Schedule {
    source: Option<String>,
    years: Years,
    days_of_week: DaysOfWeek,
    months: Months,
    days_of_month: DaysOfMonth,
    hours: Hours,
    minutes: Minutes,
    seconds: Seconds,
}

struct NextAfterQuery<Z>
where
    Z: TimeZone,
{
    initial_datetime: DateTime<Z>,
    first_month: bool,
    first_day_of_month: bool,
    first_hour: bool,
    first_minute: bool,
    first_second: bool,
}

impl<Z> NextAfterQuery<Z>
where
    Z: TimeZone,
{
    fn from(after: &DateTime<Z>) -> NextAfterQuery<Z> {
        NextAfterQuery {
            initial_datetime: after.clone() + Duration::seconds(1),
            first_month: true,
            first_day_of_month: true,
            first_hour: true,
            first_minute: true,
            first_second: true,
        }
    }

    fn year_lower_bound(&self) -> Ordinal {
        // Unlike the other units, years will never wrap around.
        self.initial_datetime.year() as u32
    }

    fn month_lower_bound(&mut self) -> Ordinal {
        if self.first_month {
            self.first_month = false;
            return self.initial_datetime.month();
        }
        Months::inclusive_min()
    }

    fn reset_month(&mut self) {
        self.first_month = false;
        self.reset_day_of_month();
    }

    fn day_of_month_lower_bound(&mut self) -> Ordinal {
        if self.first_day_of_month {
            self.first_day_of_month = false;
            return self.initial_datetime.day();
        }
        DaysOfMonth::inclusive_min()
    }

    fn reset_day_of_month(&mut self) {
        self.first_day_of_month = false;
        self.reset_hour();
    }

    fn hour_lower_bound(&mut self) -> Ordinal {
        if self.first_hour {
            self.first_hour = false;
            return self.initial_datetime.hour();
        }
        Hours::inclusive_min()
    }

    fn reset_hour(&mut self) {
        self.first_hour = false;
        self.reset_minute();
    }

    fn minute_lower_bound(&mut self) -> Ordinal {
        if self.first_minute {
            self.first_minute = false;
            return self.initial_datetime.minute();
        }
        Minutes::inclusive_min()
    }

    fn reset_minute(&mut self) {
        self.first_minute = false;
        self.reset_second();
    }

    fn second_lower_bound(&mut self) -> Ordinal {
        if self.first_second {
            self.first_second = false;
            return self.initial_datetime.second();
        }
        Seconds::inclusive_min()
    }

    fn reset_second(&mut self) {
        self.first_second = false;
    }
} // End of impl

impl Schedule {
    fn from_field_list(fields: Vec<Field>) -> Result<Schedule, Error> {
        let number_of_fields = fields.len();
        if number_of_fields != 6 && number_of_fields != 7 {
            bail!(ErrorKind::Expression(format!(
                "Expression has {} fields. Valid cron \
                 expressions have 6 or 7.",
                number_of_fields
            )));
        }

        let mut iter = fields.into_iter();

        let seconds = Seconds::from_field(iter.next().unwrap())?;
        let minutes = Minutes::from_field(iter.next().unwrap())?;
        let hours = Hours::from_field(iter.next().unwrap())?;
        let days_of_month = DaysOfMonth::from_field(iter.next().unwrap())?;
        let months = Months::from_field(iter.next().unwrap())?;
        let days_of_week = DaysOfWeek::from_field(iter.next().unwrap())?;
        let years: Years = iter
            .next()
            .map(Years::from_field)
            .unwrap_or_else(|| Ok(Years::all()))?;

        Ok(Schedule::from(
            seconds,
            minutes,
            hours,
            days_of_month,
            months,
            days_of_week,
            years,
        ))
    }

    fn from(
        seconds: Seconds,
        minutes: Minutes,
        hours: Hours,
        days_of_month: DaysOfMonth,
        months: Months,
        days_of_week: DaysOfWeek,
        years: Years,
    ) -> Schedule {
        Schedule {
            source: None,
            years: years,
            days_of_week: days_of_week,
            months: months,
            days_of_month: days_of_month,
            hours: hours,
            minutes: minutes,
            seconds: seconds,
        }
    }

    fn next_after<Z>(&self, after: &DateTime<Z>) -> Option<DateTime<Z>>
    where
        Z: TimeZone,
    {
        let mut query = NextAfterQuery::from(after);
        for year in self
            .years
            .ordinals()
            .range((Included(query.year_lower_bound()), Unbounded))
            .cloned()
        {
            let month_start = query.month_lower_bound();
            if !self.months.ordinals().contains(&month_start) {
                query.reset_month();
            }
            let month_range = (Included(month_start), Included(Months::inclusive_max()));
            for month in self.months.ordinals().range(month_range).cloned() {
                let day_of_month_start = query.day_of_month_lower_bound();
                if !self.days_of_month.ordinals().contains(&day_of_month_start) {
                    query.reset_day_of_month();
                }
                let day_of_month_end = days_in_month(month, year);
                let day_of_month_range = (Included(day_of_month_start), Included(day_of_month_end));

                'day_loop: for day_of_month in self
                    .days_of_month
                    .ordinals()
                    .range(day_of_month_range)
                    .cloned()
                {
                    let hour_start = query.hour_lower_bound();
                    if !self.hours.ordinals().contains(&hour_start) {
                        query.reset_hour();
                    }
                    let hour_range = (Included(hour_start), Included(Hours::inclusive_max()));

                    for hour in self.hours.ordinals().range(hour_range).cloned() {
                        let minute_start = query.minute_lower_bound();
                        if !self.minutes.ordinals().contains(&minute_start) {
                            query.reset_minute();
                        }
                        let minute_range =
                            (Included(minute_start), Included(Minutes::inclusive_max()));

                        for minute in self.minutes.ordinals().range(minute_range).cloned() {
                            let second_start = query.second_lower_bound();
                            if !self.seconds.ordinals().contains(&second_start) {
                                query.reset_second();
                            }
                            let second_range =
                                (Included(second_start), Included(Seconds::inclusive_max()));

                            for second in self.seconds.ordinals().range(second_range).cloned() {
                                let timezone = after.timezone();
                                let candidate = if let Some(candidate) = timezone
                                    .ymd(year as i32, month, day_of_month)
                                    .and_hms_opt(hour, minute, second)
                                {
                                    candidate
                                } else {
                                    continue;
                                };
                                if !self
                                    .days_of_week
                                    .ordinals()
                                    .contains(&candidate.weekday().number_from_sunday())
                                {
                                    continue 'day_loop;
                                }
                                return Some(candidate);
                            }
                            query.reset_minute();
                        } // End of minutes range
                        query.reset_hour();
                    } // End of hours range
                    query.reset_day_of_month();
                } // End of Day of Month range
                query.reset_month();
            } // End of Month range
        }

        // We ran out of dates to try.
        None
    }

    /// Provides an iterator which will return each DateTime that matches the schedule starting with
    /// the current time if applicable.
    pub fn upcoming<Z>(&self, timezone: Z) -> ScheduleIterator<Z>
    where
        Z: TimeZone,
    {
        self.after(&timezone.from_utc_datetime(&Utc::now().naive_utc()))
    }

    /// Like the `upcoming` method, but allows you to specify a start time other than the present.
    pub fn after<'a, Z>(&'a self, after: &DateTime<Z>) -> ScheduleIterator<'a, Z>
    where
        Z: TimeZone,
    {
        ScheduleIterator::new(self, after)
    }

    /// Returns a [TimeUnitSpec](trait.TimeUnitSpec.html) describing the years included
    /// in this [Schedule](struct.Schedule.html).
    pub fn years(&self) -> &impl TimeUnitSpec {
        &self.years
    }

    /// Returns a [TimeUnitSpec](trait.TimeUnitSpec.html) describing the months of the year included
    /// in this [Schedule](struct.Schedule.html).
    pub fn months(&self) -> &impl TimeUnitSpec {
        &self.months
    }

    /// Returns a [TimeUnitSpec](trait.TimeUnitSpec.html) describing the days of the month included
    /// in this [Schedule](struct.Schedule.html).
    pub fn days_of_month(&self) -> &impl TimeUnitSpec {
        &self.days_of_month
    }

    /// Returns a [TimeUnitSpec](trait.TimeUnitSpec.html) describing the days of the week included
    /// in this [Schedule](struct.Schedule.html).
    pub fn days_of_week(&self) -> &impl TimeUnitSpec {
        &self.days_of_week
    }

    /// Returns a [TimeUnitSpec](trait.TimeUnitSpec.html) describing the hours of the day included
    /// in this [Schedule](struct.Schedule.html).
    pub fn hours(&self) -> &impl TimeUnitSpec {
        &self.hours
    }

    /// Returns a [TimeUnitSpec](trait.TimeUnitSpec.html) describing the minutes of the hour included
    /// in this [Schedule](struct.Schedule.html).
    pub fn minutes(&self) -> &impl TimeUnitSpec {
        &self.minutes
    }

    /// Returns a [TimeUnitSpec](trait.TimeUnitSpec.html) describing the seconds of the minute included
    /// in this [Schedule](struct.Schedule.html).
    pub fn seconds(&self) -> &impl TimeUnitSpec {
        &self.seconds
    }
}

impl FromStr for Schedule {
    type Err = Error;
    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        match schedule(Input(expression)) {
            Ok((_, mut schedule)) => {
                schedule.source.replace(expression.to_owned());
                Ok(schedule)
            }, // Extract from nom tuple
            Err(_) => bail!(ErrorKind::Expression("Invalid cron expression.".to_owned())), //TODO: Details
        }
    }
}

impl From<Schedule> for String {
    fn from(schedule: Schedule) -> String {
        schedule.source.unwrap()
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.source.as_ref().map(|s| write!(f, "{}", s)).unwrap()
    }
}

pub struct ScheduleIterator<'a, Z>
where
    Z: TimeZone,
{
    is_done: bool,
    schedule: &'a Schedule,
    previous_datetime: DateTime<Z>,
}
//TODO: Cutoff datetime?

impl<'a, Z> ScheduleIterator<'a, Z>
where
    Z: TimeZone,
{
    fn new(schedule: &'a Schedule, starting_datetime: &DateTime<Z>) -> ScheduleIterator<'a, Z> {
        ScheduleIterator {
            is_done: false,
            schedule: schedule,
            previous_datetime: starting_datetime.clone(),
        }
    }
}

impl<'a, Z> Iterator for ScheduleIterator<'a, Z>
where
    Z: TimeZone,
{
    type Item = DateTime<Z>;

    fn next(&mut self) -> Option<DateTime<Z>> {
        if self.is_done {
            return None;
        }
        if let Some(next_datetime) = self.schedule.next_after(&self.previous_datetime) {
            self.previous_datetime = next_datetime.clone();
            Some(next_datetime)
        } else {
            self.is_done = true;
            None
        }
    }
}

pub type Ordinal = u32;
// TODO: Make OrdinalSet an enum.
// It should either be a BTreeSet of ordinals or an `All` option to save space.
// `All` can iterate from inclusive_min to inclusive_max and answer membership
// queries
pub type OrdinalSet = BTreeSet<Ordinal>;

#[derive(Debug)]
pub enum Specifier {
    All,
    Point(Ordinal),
    NamedPoint(String),
    Period(Ordinal, u32),
    Range(Ordinal, Ordinal),
    NamedRange(String, String),
}

#[derive(Debug)]
pub struct Field {
    pub specifiers: Vec<Specifier>, // TODO: expose iterator?
}

trait FromField
where
    Self: Sized,
{
    //TODO: Replace with std::convert::TryFrom when stable
    fn from_field(field: Field) -> Result<Self, Error>;
}

impl<T> FromField for T
where
    T: TimeUnitField,
{
    fn from_field(field: Field) -> Result<T, Error> {
        let mut ordinals = OrdinalSet::new(); //TODO: Combinator
        for specifier in field.specifiers {
            let specifier_ordinals: OrdinalSet = T::ordinals_from_specifier(&specifier)?;
            for ordinal in specifier_ordinals {
                ordinals.insert(T::validate_ordinal(ordinal)?);
            }
        }

        Ok(T::from_ordinal_set(ordinals))
    }
}

named!(
    ordinal<Input, u32>,
    map_res!(ws!(digit), |x: Input| x.0.parse())
);

named!(
    name<Input, String>,
    map!(ws!(alpha), |x| x.0.to_owned())
);

named!(
    point<Input, Specifier>,
    do_parse!(o: ordinal >> (Specifier::Point(o)))
);

named!(
    named_point<Input, Specifier>,
    do_parse!(n: name >> (Specifier::NamedPoint(n)))
);

named!(
    period<Input, Specifier>,
    complete!(do_parse!(
        start: ordinal >> tag!("/") >> step: ordinal >> (Specifier::Period(start, step))
    ))
);

named!(
    range<Input, Specifier>,
    complete!(do_parse!(
        start: ordinal >> tag!("-") >> end: ordinal >> (Specifier::Range(start, end))
    ))
);

named!(
    named_range<Input, Specifier>,
    complete!(do_parse!(
        start: name >> tag!("-") >> end: name >> (Specifier::NamedRange(start, end))
    ))
);

named!(all<Input, Specifier>, do_parse!(tag!("*") >> (Specifier::All)));

named!(
    specifier<Input, Specifier>,
    alt!(all | period | range | point | named_range | named_point)
);

named!(
    specifier_list<Input, Vec<Specifier>>,
    ws!(alt!(
        do_parse!(list: separated_nonempty_list!(tag!(","), specifier) >> (list))
            | do_parse!(spec: specifier >> (vec![spec]))
    ))
);

named!(
    field<Input, Field>,
    do_parse!(specifiers: specifier_list >> (Field { specifiers }))
);

named!(
    shorthand_yearly<Input, Schedule>,
    do_parse!(
        tag!("@yearly")
            >> (Schedule::from(
                Seconds::from_ordinal(0),
                Minutes::from_ordinal(0),
                Hours::from_ordinal(0),
                DaysOfMonth::from_ordinal(1),
                Months::from_ordinal(1),
                DaysOfWeek::all(),
                Years::all()
            ))
    )
);

named!(
    shorthand_monthly<Input, Schedule>,
    do_parse!(
        tag!("@monthly")
            >> (Schedule::from(
                Seconds::from_ordinal_set(iter::once(0).collect()),
                Minutes::from_ordinal_set(iter::once(0).collect()),
                Hours::from_ordinal_set(iter::once(0).collect()),
                DaysOfMonth::from_ordinal_set(iter::once(1).collect()),
                Months::all(),
                DaysOfWeek::all(),
                Years::all()
            ))
    )
);

named!(
    shorthand_weekly<Input, Schedule>,
    do_parse!(
        tag!("@weekly")
            >> (Schedule::from(
                Seconds::from_ordinal_set(iter::once(0).collect()),
                Minutes::from_ordinal_set(iter::once(0).collect()),
                Hours::from_ordinal_set(iter::once(0).collect()),
                DaysOfMonth::all(),
                Months::all(),
                DaysOfWeek::from_ordinal_set(iter::once(1).collect()),
                Years::all()
            ))
    )
);

named!(
    shorthand_daily<Input, Schedule>,
    do_parse!(
        tag!("@daily")
            >> (Schedule::from(
                Seconds::from_ordinal_set(iter::once(0).collect()),
                Minutes::from_ordinal_set(iter::once(0).collect()),
                Hours::from_ordinal_set(iter::once(0).collect()),
                DaysOfMonth::all(),
                Months::all(),
                DaysOfWeek::all(),
                Years::all()
            ))
    )
);

named!(
    shorthand_hourly<Input, Schedule>,
    do_parse!(
        tag!("@hourly")
            >> (Schedule::from(
                Seconds::from_ordinal_set(iter::once(0).collect()),
                Minutes::from_ordinal_set(iter::once(0).collect()),
                Hours::all(),
                DaysOfMonth::all(),
                Months::all(),
                DaysOfWeek::all(),
                Years::all()
            ))
    )
);

named!(
    shorthand<Input, Schedule>,
    alt!(
        shorthand_yearly
            | shorthand_monthly
            | shorthand_weekly
            | shorthand_daily
            | shorthand_hourly
    )
);

named!(
    longhand<Input, Schedule>,
    map_res!(
        complete!(do_parse!(
            fields: many_m_n!(6, 7, field) >> eof!() >> (fields)
        )),
        Schedule::from_field_list
    )
);

named!(schedule<Input, Schedule>, alt!(shorthand | longhand));

fn is_leap_year(year: Ordinal) -> bool {
    let by_four = year % 4 == 0;
    let by_hundred = year % 100 == 0;
    let by_four_hundred = year % 400 == 0;
    by_four && ((!by_hundred) || by_four_hundred)
}

fn days_in_month(month: Ordinal, year: Ordinal) -> u32 {
    let is_leap_year = is_leap_year(year);
    match month {
        9 | 4 | 6 | 11 => 30,
        2 if is_leap_year => 29,
        2 => 28,
        _ => 31,
    }
}

#[test]
fn test_next_after() {
    let expression = "0 5,13,40-42 17 1 Jan *";
    let schedule = schedule(Input(expression)).unwrap().1;
    let next = schedule.next_after(&Utc::now());
    println!("NEXT AFTER for {} {:?}", expression, next);
    assert!(next.is_some());
}

#[test]
fn test_upcoming_utc() {
    let expression = "0 0,30 0,6,12,18 1,15 Jan-March Thurs";
    let schedule = schedule(Input(expression)).unwrap().1;
    let mut upcoming = schedule.upcoming(Utc);
    let next1 = upcoming.next();
    assert!(next1.is_some());
    let next2 = upcoming.next();
    assert!(next2.is_some());
    let next3 = upcoming.next();
    assert!(next3.is_some());
    println!("Upcoming 1 for {} {:?}", expression, next1);
    println!("Upcoming 2 for {} {:?}", expression, next2);
    println!("Upcoming 3 for {} {:?}", expression, next3);
}

#[test]
fn test_upcoming_local() {
    use chrono::Local;
    let expression = "0 0,30 0,6,12,18 1,15 Jan-March Thurs";
    let schedule = schedule(Input(expression)).unwrap().1;
    let mut upcoming = schedule.upcoming(Local);
    let next1 = upcoming.next();
    assert!(next1.is_some());
    let next2 = upcoming.next();
    assert!(next2.is_some());
    let next3 = upcoming.next();
    assert!(next3.is_some());
    println!("Upcoming 1 for {} {:?}", expression, next1);
    println!("Upcoming 2 for {} {:?}", expression, next2);
    println!("Upcoming 3 for {} {:?}", expression, next3);
}

#[test]
fn test_schedule_to_string() {
    let expression = "* 1,2,3 * * * *";
    let schedule: Schedule = Schedule::from_str(expression).unwrap();
    let result = String::from(schedule);
    assert_eq!(expression, result);
}

#[test]
fn test_display_schedule() {
    use std::fmt::Write;
    let expression = "@monthly";
    let schedule = Schedule::from_str(expression).unwrap();
    let mut result = String::new();
    write!(result, "{}", schedule).unwrap();
    assert_eq!(expression, result);
}

#[test]
fn test_valid_from_str() {
    let schedule = Schedule::from_str("0 0,30 0,6,12,18 1,15 Jan-March Thurs");
    schedule.unwrap();
}

#[test]
fn test_invalid_from_str() {
    let schedule = Schedule::from_str("cheesecake 0,30 0,6,12,18 1,15 Jan-March Thurs");
    assert!(schedule.is_err());
}

#[test]
fn test_nom_valid_number() {
    let expression = "1997";
    point(Input(expression)).unwrap();
}

#[test]
fn test_nom_invalid_point() {
    let expression = "a";
    assert!(point(Input(expression)).is_err());
}

#[test]
fn test_nom_valid_named_point() {
    let expression = "WED";
    named_point(Input(expression)).unwrap();
}

#[test]
fn test_nom_invalid_named_point() {
    let expression = "8";
    assert!(named_point(Input(expression)).is_err());
}

#[test]
fn test_nom_valid_period() {
    let expression = "1/2";
    period(Input(expression)).unwrap();
}

#[test]
fn test_nom_invalid_period() {
    let expression = "Wed/4";
    assert!(period(Input(expression)).is_err());
}

#[test]
fn test_nom_valid_number_list() {
    let expression = "1,2";
    field(Input(expression)).unwrap();
}

#[test]
fn test_nom_invalid_number_list() {
    let expression = ",1,2";
    assert!(field(Input(expression)).is_err());
}

#[test]
fn test_nom_valid_range_field() {
    let expression = "1-4";
    range(Input(expression)).unwrap();
}

#[test]
fn test_nom_invalid_range_field() {
    let expression = "-4";
    assert!(range(Input(expression)).is_err());
}

#[test]
fn test_nom_valid_named_range_field() {
    let expression = "TUES-THURS";
    named_range(Input(expression)).unwrap();
}

#[test]
fn test_nom_invalid_named_range_field() {
    let expression = "3-THURS";
    assert!(named_range(Input(expression)).is_err());
}

#[test]
fn test_nom_valid_schedule() {
    let expression = "* * * * * *";
    schedule(Input(expression)).unwrap();
}

#[test]
fn test_nom_invalid_schedule() {
    let expression = "* * * *";
    assert!(schedule(Input(expression)).is_err());
}

#[test]
fn test_nom_valid_seconds_list() {
    let expression = "0,20,40 * * * * *";
    schedule(Input(expression)).unwrap();
}

#[test]
fn test_nom_valid_seconds_range() {
    let expression = "0-40 * * * * *";
    schedule(Input(expression)).unwrap();
}

#[test]
fn test_nom_valid_seconds_mix() {
    let expression = "0-5,58 * * * * *";
    schedule(Input(expression)).unwrap();
}

#[test]
fn test_nom_invalid_seconds_range() {
    let expression = "0-65 * * * * *";
    assert!(schedule(Input(expression)).is_err());
}

#[test]
fn test_nom_invalid_seconds_list() {
    let expression = "103,12 * * * * *";
    assert!(schedule(Input(expression)).is_err());
}

#[test]
fn test_nom_invalid_seconds_mix() {
    let expression = "0-5,102 * * * * *";
    assert!(schedule(Input(expression)).is_err());
}

#[test]
fn test_nom_valid_days_of_week_list() {
    let expression = "* * * * * MON,WED,FRI";
    schedule(Input(expression)).unwrap();
}

#[test]
fn test_nom_invalid_days_of_week_list() {
    let expression = "* * * * * MON,TURTLE";
    assert!(schedule(Input(expression)).is_err());
}

#[test]
fn test_nom_valid_days_of_week_range() {
    let expression = "* * * * * MON-FRI";
    schedule(Input(expression)).unwrap();
}

#[test]
fn test_nom_invalid_days_of_week_range() {
    let expression = "* * * * * BEAR-OWL";
    assert!(schedule(Input(expression)).is_err());
}

#[test]
fn test_no_panic_on_nonexistent_time() {
    use chrono::offset::TimeZone;
    use chrono_tz::Tz;

    let schedule_tz: Tz = "Europe/London".parse().unwrap();
    let dt = schedule_tz
        .ymd(2019, 10, 27)
        .and_hms(0, 3, 29)
        .checked_add_signed(chrono::Duration::hours(1)) // puts it in the middle of the DST transition
        .unwrap();
    let schedule = Schedule::from_str("* * * * * Sat,Sun *").unwrap();
    let next = schedule.after(&dt).next().unwrap();
    assert!(next > dt); // test is ensuring line above does not panic
}
