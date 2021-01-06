extern crate chrono;
extern crate cron;

#[cfg(test)]
mod tests {
    use chrono::*;
    use cron::{Schedule, TimeUnitSpec};
    use std::collections::Bound::{Excluded, Included};
    use std::str::FromStr;

    #[test]
    fn test_readme() {
        let expression = "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
        let schedule = Schedule::from_str(expression).unwrap();
        println!("README: Upcoming fire times for '{}':", expression);
        for datetime in schedule.upcoming(Utc).take(10) {
            println!("README: -> {}", datetime);
        }
    }

    #[test]
    fn test_anything_goes() {
        let expression = "* * * * * * *";
        let schedule = Schedule::from_str(expression).unwrap();
        println!("All stars: Upcoming fire times for '{}':", expression);
        for datetime in schedule.upcoming(Utc).take(10) {
            println!("All stars: -> {}", datetime);
        }
    }

    #[test]
    fn test_parse_with_year() {
        let expression = "1 2 3 4 5 6 2015";
        assert!(Schedule::from_str(expression).is_ok());
    }

    #[test]
    fn test_parse_with_seconds_list() {
        let expression = "1,30,40 2 3 4 5 Mon-Fri";
        assert!(Schedule::from_str(expression).is_ok());
    }

    #[test]
    fn test_parse_with_lists() {
        let expression = "1 2,17,51 1-3,6,9-11 4,29 2,3,7 Tues";
        let schedule = Schedule::from_str(expression).unwrap();
        let mut date = Utc::now();
        println!("Fire times for {}:", expression);
        for _ in 0..20 {
            date = schedule.after(&date).next().expect("No further dates!");
            println!("-> {}", date);
        }
        assert!(true);
    }

    #[test]
    fn test_upcoming_iterator() {
        let expression = "0 2,17,51 1-3,6,9-11 4,29 2,3,7 Wed";
        let schedule = Schedule::from_str(expression).unwrap();
        println!("Upcoming fire times for '{}':", expression);
        for datetime in schedule.upcoming(Utc).take(12) {
            println!("-> {}", datetime);
        }
        assert!(true);
    }

    #[test]
    fn test_parse_without_year() {
        let expression = "1 2 3 4 5 6";
        assert!(Schedule::from_str(expression).is_ok());
    }

    #[test]
    fn test_parse_too_many_fields() {
        let expression = "1 2 3 4 5 6 7 8 9 2019";
        assert!(Schedule::from_str(expression).is_err());
    }

    #[test]
    fn test_not_enough_fields() {
        let expression = "1 2 3 2019";
        assert!(Schedule::from_str(expression).is_err());
    }

    #[test]
    fn test_next_utc() {
        let expression = "1 2 3 4 10 Fri";
        let schedule = Schedule::from_str(expression).unwrap();
        let next = schedule
            .upcoming(Utc)
            .next()
            .expect("There was no upcoming fire time.");
        println!("Next fire time: {}", next.to_rfc3339());
    }

    #[test]
    fn test_yearly() {
        let expression = "@yearly";
        let schedule = Schedule::from_str(expression).expect("Failed to parse @yearly.");
        let starting_date = Utc.ymd(2017, 6, 15).and_hms(14, 29, 36);
        let mut events = schedule.after(&starting_date);
        assert_eq!(Utc.ymd(2018, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
        assert_eq!(Utc.ymd(2019, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
        assert_eq!(Utc.ymd(2020, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
    }

    #[test]
    fn test_monthly() {
        let expression = "@monthly";
        let schedule = Schedule::from_str(expression).expect("Failed to parse @monthly.");
        let starting_date = Utc.ymd(2017, 10, 15).and_hms(14, 29, 36);
        let mut events = schedule.after(&starting_date);
        assert_eq!(
            Utc.ymd(2017, 11, 1).and_hms(0, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2017, 12, 1).and_hms(0, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(Utc.ymd(2018, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
    }

    #[test]
    fn test_weekly() {
        let expression = "@weekly";
        let schedule = Schedule::from_str(expression).expect("Failed to parse @weekly.");
        let starting_date = Utc.ymd(2016, 12, 23).and_hms(14, 29, 36);
        let mut events = schedule.after(&starting_date);
        assert_eq!(
            Utc.ymd(2016, 12, 25).and_hms(0, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(Utc.ymd(2017, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
        assert_eq!(Utc.ymd(2017, 1, 8).and_hms(0, 0, 0), events.next().unwrap());
    }

    #[test]
    fn test_daily() {
        let expression = "@daily";
        let schedule = Schedule::from_str(expression).expect("Failed to parse @daily.");
        let starting_date = Utc.ymd(2016, 12, 29).and_hms(14, 29, 36);
        let mut events = schedule.after(&starting_date);
        assert_eq!(
            Utc.ymd(2016, 12, 30).and_hms(0, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2016, 12, 31).and_hms(0, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(Utc.ymd(2017, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
    }

    #[test]
    fn test_hourly() {
        let expression = "@hourly";
        let schedule = Schedule::from_str(expression).expect("Failed to parse @hourly.");
        let starting_date = Utc.ymd(2017, 2, 25).and_hms(22, 29, 36);
        let mut events = schedule.after(&starting_date);
        assert_eq!(
            Utc.ymd(2017, 2, 25).and_hms(23, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2017, 2, 26).and_hms(0, 0, 0),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2017, 2, 26).and_hms(1, 0, 0),
            events.next().unwrap()
        );
    }

    #[test]
    fn test_step_schedule() {
        let expression = "0/20 0/5 0 1 1 * *";
        let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");
        let starting_date = Utc.ymd(2017, 6, 15).and_hms(14, 29, 36);
        let mut events = schedule.after(&starting_date);

        assert_eq!(Utc.ymd(2018, 1, 1).and_hms(0, 0, 0), events.next().unwrap());
        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 0, 20),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 0, 40),
            events.next().unwrap()
        );

        assert_eq!(Utc.ymd(2018, 1, 1).and_hms(0, 5, 0), events.next().unwrap());
        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 5, 20),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 5, 40),
            events.next().unwrap()
        );

        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 10, 0),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 10, 20),
            events.next().unwrap()
        );
        assert_eq!(
            Utc.ymd(2018, 1, 1).and_hms(0, 10, 40),
            events.next().unwrap()
        );
    }

    #[test]
    fn test_time_unit_spec_years() {
        let expression = "* * * * * * 2015-2044";
        let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");

        // Membership
        assert_eq!(true, schedule.years().includes(2031));
        assert_eq!(false, schedule.years().includes(1969));

        // Number of years specified
        assert_eq!(30, schedule.years().count());

        // Iterator
        let mut years_iter = schedule.years().iter();
        assert_eq!(Some(2015), years_iter.next());
        assert_eq!(Some(2016), years_iter.next());
        // ...

        // Range Iterator
        let mut five_year_plan = schedule.years().range((Included(2017), Excluded(2017 + 5)));
        assert_eq!(Some(2017), five_year_plan.next());
        assert_eq!(Some(2018), five_year_plan.next());
        assert_eq!(Some(2019), five_year_plan.next());
        assert_eq!(Some(2020), five_year_plan.next());
        assert_eq!(Some(2021), five_year_plan.next());
        assert_eq!(None, five_year_plan.next());
    }

    #[test]
    fn test_time_unit_spec_months() {
        let expression = "* * * * 5-8 * *";
        let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");

        // Membership
        assert_eq!(false, schedule.months().includes(4));
        assert_eq!(true, schedule.months().includes(6));

        // Iterator
        let mut summer = schedule.months().iter();
        assert_eq!(Some(5), summer.next());
        assert_eq!(Some(6), summer.next());
        assert_eq!(Some(7), summer.next());
        assert_eq!(Some(8), summer.next());
        assert_eq!(None, summer.next());

        // Number of months specified
        assert_eq!(4, schedule.months().count());

        // Range Iterator
        let mut first_half_of_summer = schedule.months().range((Included(1), Included(6)));
        assert_eq!(Some(5), first_half_of_summer.next());
        assert_eq!(Some(6), first_half_of_summer.next());
        assert_eq!(None, first_half_of_summer.next());
    }

    #[test]
    fn test_time_unit_spec_days_of_month() {
        let expression = "* * * 1,15 * * *";
        let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");
        // Membership
        assert_eq!(true, schedule.days_of_month().includes(1));
        assert_eq!(false, schedule.days_of_month().includes(7));

        // Iterator
        let mut paydays = schedule.days_of_month().iter();
        assert_eq!(Some(1), paydays.next());
        assert_eq!(Some(15), paydays.next());
        assert_eq!(None, paydays.next());

        // Number of years specified
        assert_eq!(2, schedule.days_of_month().count());

        // Range Iterator
        let mut mid_month_paydays = schedule.days_of_month().range((Included(5), Included(25)));
        assert_eq!(Some(15), mid_month_paydays.next());
        assert_eq!(None, mid_month_paydays.next());
    }

    #[test]
    fn test_first_ordinals_not_in_set_1() {
        let schedule = "0 0/10 * * * * *".parse::<Schedule>().unwrap();
        let start_time_1 = NaiveDate::from_ymd(2017, 10, 24).and_hms(0, 0, 59);
        let start_time_1 = Utc.from_utc_datetime(&start_time_1);
        let next_time_1 = schedule.after(&start_time_1).next().unwrap();

        let start_time_2 = NaiveDate::from_ymd(2017, 10, 24).and_hms(0, 1, 0);
        let start_time_2 = Utc.from_utc_datetime(&start_time_2);
        let next_time_2 = schedule.after(&start_time_2).next().unwrap();
        assert_eq!(next_time_1, next_time_2);
    }

    #[test]
    fn test_first_ordinals_not_in_set_2() {
        let schedule_1 = "00 00 23 * * * *".parse::<Schedule>().unwrap();
        let start_time = NaiveDate::from_ymd(2018, 11, 15).and_hms(22, 30, 00);
        let start_time = Utc.from_utc_datetime(&start_time);
        let next_time_1 = schedule_1.after(&start_time).next().unwrap();

        let schedule_2 = "00 00 * * * * *".parse::<Schedule>().unwrap();
        let next_time_2 = schedule_2.after(&start_time).next().unwrap();
        assert_eq!(next_time_1, next_time_2);
    }
}
