
/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';
import { AppState } from '@/app/store';
import { Store } from 'vuex';
import {
  CalendarEvent, CreateEvent, DeleteEvent, GetEvents, UpdateEvent,
} from './model';
import { Commands, Queries } from './routes';


export class CalendarService {
  private apiClient: ApiClient;
  private store: Store<AppState>;

  constructor(apiClient: ApiClient, store: Store<AppState>) {
    this.apiClient = apiClient;
    this.store = store;
  }

  async createEvent(input: CreateEvent): Promise<CalendarEvent> {
    const res: CalendarEvent = await this.apiClient.post(Commands.createEvent, input);

    return res;
  }

  async deleteEvent(input: DeleteEvent): Promise<void> {
    await this.apiClient.post(Commands.deleteEvent, input);
  }

  async fetchEvents(input: GetEvents): Promise<CalendarEvent[]> {
    const res: CalendarEvent[] = await this.apiClient.post(Queries.events, input);

    return res;
  }

  async updateEvent(input: UpdateEvent): Promise<CalendarEvent> {
    const res: CalendarEvent = await this.apiClient.post(Commands.updateEvent, input);

    return res;
  }
}

export const CalendarServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: CalendarService) {
    Vue.prototype.$calendarService = service;
  },
};
