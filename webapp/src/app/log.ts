/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable */


function deepClone(obj: any): any {
  let copy: any = undefined;

  // Handle the 3 simple types, and null or undefined
  if (null == obj || "object" != typeof obj) return obj;

  // Handle Date
  if (obj instanceof Date) {
      copy = new Date();
      copy.setTime(obj.getTime());
      return copy;
  }

  // Handle Array
  if (obj instanceof Array) {
      copy = [];
      for (var i = 0, len = obj.length; i < len; i++) {
          copy[i] = deepClone(obj[i]);
      }
      return copy;
  }

  // Handle Object
  if (obj instanceof Object) {
      copy = {};
      for (var attr in obj) {
          if (obj.hasOwnProperty(attr)) {
            copy[attr] = deepClone(obj[attr]);
          }
      }
      return copy;
  }

  throw new Error("Unable to copy obj! Its type isn't supported.");
}
/* eslint-enable */


export enum LogLevel {
  DEBUG,
  INFO,
  WARN,
  ERROR,
  FATAL,
  NONE,
  NOOP,
}

export type Options = {
  level?: LogLevel;
  // eslint-disable-next-line @typescript-eslint/ban-types
  fields?: Object;
  timestampFieldName?: string;
  messageFieldName?: string;
  levelFieldName?: string;
  // eslint-disable-next-line @typescript-eslint/ban-types
  hooks?: { (event: Object): void }[];
};

export interface LoggerInterface {
  config(options: Options): void;
  // eslint-disable-next-line @typescript-eslint/ban-types
  clone(fields: Object): LoggerInterface;
  debug(message: string): void;
  info(message: string): void;
  warn(message: string): void;
  error(message: string): void;
  fatal(message: string): void; // log with the "fatal" level then exit(1)
  // eslint-disable-next-line @typescript-eslint/ban-types
  track(fields: Object): void;
  log(loverl: LogLevel, message: string | Error | null): void;

}

export class Logger implements LoggerInterface {
  private level: LogLevel = LogLevel.DEBUG;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  private fields: any = {};
  private writer = console;
  private insertTimestamp = true;
  private timestampFieldName = 'timestamp';
  private messageFieldName = 'message';
  private levelFieldName = 'level';
  private errorFieldName = 'error';
  // eslint-disable-next-line @typescript-eslint/ban-types
  private hooks: { (event: Object): void }[] = [];


  constructor(options?: Options) {
    if (options) {
      this.config(options);
    }
  }

  /**
   * configure the logger with the given options
   */
  config(options: Options): void {
    if (options.fields) {
      this.fields = options.fields;
    }
    if (options.level) {
      this.level = options.level;
    }
    if (options.timestampFieldName) {
      this.timestampFieldName = options.timestampFieldName;
    }
    if (options.messageFieldName) {
      this.messageFieldName = options.messageFieldName;
    }
    if (options.levelFieldName) {
      this.levelFieldName = options.levelFieldName;
    }
    if (options.hooks) {
      this.hooks = options.hooks;
    }
  }

  /**
   * create a copy of the logger, add the given fields to it and return it
   */
  // eslint-disable-next-line @typescript-eslint/ban-types
  clone(fields: Object): Logger {
    const newLogger = Object.create(this);
    newLogger.fields = { ...this.fields, ...fields };
    return newLogger;
  }

  /**
   * log an event with the DEBUG level
   */
  debug(message: string): void {
    this.log(LogLevel.DEBUG, message);
  }

  /**
   * log an event with the INFO level
   */
  info(message: string): void {
    this.log(LogLevel.INFO, message);
  }

  /**
   * log an event with the WARN level
   */
  warn(message: string): void {
    this.log(LogLevel.WARN, message);
  }

  /**
   * log an event with the ERROR level
   */
  error(message: string): void {
    this.log(LogLevel.ERROR, message);
  }

  /**
   * log an event with the FATAL level then exit(1)
   */
  fatal(message: string): void {
    this.log(LogLevel.FATAL, message);
    throw new Error(message);
  }

  /**
   * log an event without level nor message
   * @param {Object} [fields] - additional fields to add to the event (optional)
   */
  // eslint-disable-next-line @typescript-eslint/ban-types
  track(fields: Object): void {
    const newLogger = this.clone(fields);
    newLogger.log(LogLevel.NONE, null);
  }

  log(level: LogLevel, message: string | Error | null) {
    if (level < this.level) {
      return;
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const event: any = deepClone(this.fields);

    // handle message
    if (message === undefined || message === null) {
      // do nothing
    } else if (typeof message === 'string' && message.length > 0) {
      event[this.messageFieldName] = message;
    } else if (message instanceof Error) {
      // eslint-disable-next-line @typescript-eslint/camelcase
      event.error_name = message.name;
      event[this.messageFieldName] = message.message;
    } else {
      event[this.messageFieldName] = JSON.stringify(message);
    }

    // handle timestamp
    if (this.insertTimestamp === true) {
      event[this.timestampFieldName] = new Date().toISOString();
    }

    // default case: do not insert level field
    switch (level) {
      case LogLevel.DEBUG:
        event[this.levelFieldName] = 'debug';
        break;
      case LogLevel.INFO:
        event[this.levelFieldName] = 'info';
        break;
      case LogLevel.WARN:
        event[this.levelFieldName] = 'warning';
        break;
      case LogLevel.ERROR:
        event[this.levelFieldName] = 'error';
        break;
      case LogLevel.FATAL:
        event[this.levelFieldName] = 'fatal';
        break;
      default:
        break;
    }

    this.hooks.forEach((hook) => hook(event));

    this.writer.log(event);
  }
}
