use crate::Solar;
use crate::event::{Event, EventQuery, scan_events_in_range, scan_events_in_range_filtered};

pub trait CalendarDay {
    fn solar(&self) -> Solar;

    fn events(&self) -> Vec<Event> {
        self.solar().events()
    }

    fn all_events(&self) -> Vec<Event> {
        self.solar().all_events()
    }

    fn find_events(&self, query: &EventQuery<'_>) -> Vec<Event> {
        self.solar().find_events(query)
    }

    fn events_until(&self, end: Solar) -> Vec<Event> {
        scan_events_in_range(self.solar(), end)
    }

    fn find_events_until(&self, end: Solar, query: &EventQuery<'_>) -> Vec<Event> {
        scan_events_in_range_filtered(self.solar(), end, query)
    }
}

pub trait CalendarSpan {
    fn first_solar_day(&self) -> Solar;
    fn last_solar_day(&self) -> Solar;

    fn contains_solar(&self, solar: Solar) -> bool {
        let start = self.first_solar_day();
        let end = self.last_solar_day();
        solar.subtract(&start) >= 0 && end.subtract(&solar) >= 0
    }

    fn events(&self) -> Vec<Event> {
        scan_events_in_range(self.first_solar_day(), self.last_solar_day())
    }

    fn all_events(&self) -> Vec<Event> {
        self.events()
    }

    fn find_events(&self, query: &EventQuery<'_>) -> Vec<Event> {
        scan_events_in_range_filtered(self.first_solar_day(), self.last_solar_day(), query)
    }

    fn events_until(&self, end: Solar) -> Vec<Event> {
        scan_events_in_range(self.first_solar_day(), end)
    }

    fn find_events_until(&self, end: Solar, query: &EventQuery<'_>) -> Vec<Event> {
        scan_events_in_range_filtered(self.first_solar_day(), end, query)
    }
}

pub(crate) fn point_events<T: CalendarDay>(value: &T) -> Vec<Event> {
    CalendarDay::events(value)
}

pub(crate) fn point_all_events<T: CalendarDay>(value: &T) -> Vec<Event> {
    CalendarDay::all_events(value)
}

pub(crate) fn point_find_events<T: CalendarDay>(value: &T, query: &EventQuery<'_>) -> Vec<Event> {
    CalendarDay::find_events(value, query)
}

pub(crate) fn point_events_until<T: CalendarDay>(value: &T, end: Solar) -> Vec<Event> {
    CalendarDay::events_until(value, end)
}

pub(crate) fn point_find_events_until<T: CalendarDay>(value: &T, end: Solar, query: &EventQuery<'_>) -> Vec<Event> {
    CalendarDay::find_events_until(value, end, query)
}

pub(crate) fn span_contains_solar<T: CalendarSpan>(value: &T, solar: Solar) -> bool {
    CalendarSpan::contains_solar(value, solar)
}

pub(crate) fn span_events<T: CalendarSpan>(value: &T) -> Vec<Event> {
    CalendarSpan::events(value)
}

pub(crate) fn span_all_events<T: CalendarSpan>(value: &T) -> Vec<Event> {
    CalendarSpan::all_events(value)
}

pub(crate) fn span_find_events<T: CalendarSpan>(value: &T, query: &EventQuery<'_>) -> Vec<Event> {
    CalendarSpan::find_events(value, query)
}

pub(crate) fn span_events_until<T: CalendarSpan>(value: &T, end: Solar) -> Vec<Event> {
    CalendarSpan::events_until(value, end)
}

pub(crate) fn span_find_events_until<T: CalendarSpan>(value: &T, end: Solar, query: &EventQuery<'_>) -> Vec<Event> {
    CalendarSpan::find_events_until(value, end, query)
}
