CREATE TABLE calendar_events (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  title TEXT NOT NULL,
  description TEXT NOT NULL,
  location TEXT NOT NULL,
  start_at TIMESTAMP WITH TIME ZONE NOT NULL,
  end_at TIMESTAMP WITH TIME ZONE NOT NULL,

  namespace_id UUID NOT NULL REFERENCES kernel_namespaces (id) ON DELETE CASCADE
);
CREATE INDEX index_calendar_events_on_namespace_id ON calendar_events (namespace_id);
CREATE INDEX index_calendar_events_on_start_at ON calendar_events (start_at);
CREATE INDEX index_calendar_events_on_end_at ON calendar_events (end_at);
