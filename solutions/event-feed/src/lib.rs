use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Eq)]
pub struct Event {
    pub timestamp: u64,
    pub source: String,
    pub message: String,
}

/// Merges two timestamp-ordered streams, yielding owned events one at a time.
pub struct EventFeed {
    primary: Peekable<IntoIter<Event>>,
    secondary: Peekable<IntoIter<Event>>,
}

impl EventFeed {
    /// Rejects either input if a timestamp is smaller than the one before it.
    /// Equal timestamps within one input are valid.
    pub fn new(primary: Vec<Event>, secondary: Vec<Event>) -> Result<Self, String> {
        for (index, window) in primary.windows(2).enumerate() {
            if window[1].timestamp < window[0].timestamp {
                return Err(format!("primary event {} is out of order", index + 2));
            }
        }
        for (index, window) in secondary.windows(2).enumerate() {
            if window[1].timestamp < window[0].timestamp {
                return Err(format!("secondary event {} is out of order", index + 2));
            }
        }
        Ok(EventFeed {
            primary: primary.into_iter().peekable(),
            secondary: secondary.into_iter().peekable(),
        })
    }
}

impl Iterator for EventFeed {
    type Item = Event;

    /// On a tie, yields the primary event first. Keeps each input's order.
    fn next(&mut self) -> Option<Self::Item> {
        match (self.primary.peek(), self.secondary.peek()) {
            (None, None) => None,
            (Some(_), None) => self.primary.next(),
            (None, Some(_)) => self.secondary.next(),
            (Some(pe), Some(se)) => {
                if se.timestamp < pe.timestamp {
                    return self.secondary.next();
                } else {
                    return self.primary.next();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(timestamp: u64, source: &str, message: &str) -> Event {
        Event {
            timestamp,
            source: source.into(),
            message: message.into(),
        }
    }

    // Construct a feed directly so the iteration tests can be solved before validation.
    fn sorted_feed(primary: Vec<Event>, secondary: Vec<Event>) -> EventFeed {
        EventFeed {
            primary: primary.into_iter().peekable(),
            secondary: secondary.into_iter().peekable(),
        }
    }

    #[test]
    fn constructor_accepts_sorted_inputs_and_equal_timestamps() {
        let feed = EventFeed::new(
            vec![event(2, "app", "started"), event(2, "app", "ready")],
            vec![event(1, "db", "connected")],
        )
        .expect("both inputs are ordered");
        assert_eq!(feed.count(), 3);
    }

    #[test]
    fn constructor_rejects_an_out_of_order_input() {
        let primary = vec![event(3, "app", "later"), event(2, "app", "earlier")];
        let error = match EventFeed::new(primary, vec![]) {
            Ok(_) => panic!("primary should have been rejected"),
            Err(error) => error,
        };
        assert!(error.contains("primary"), "{error}");
        assert!(error.contains('2'), "{error}");

        let secondary = vec![event(1, "db", "first"), event(0, "db", "late arrival")];
        let error = match EventFeed::new(vec![], secondary) {
            Ok(_) => panic!("secondary should have been rejected"),
            Err(error) => error,
        };
        assert!(error.contains("secondary"), "{error}");
        assert!(error.contains('2'), "{error}");
    }

    #[test]
    fn merges_in_time_order_and_preserves_ties() {
        let feed = sorted_feed(
            vec![
                event(2, "app", "started"),
                event(2, "app", "ready"),
                event(5, "app", "served"),
            ],
            vec![event(1, "db", "connected"), event(2, "db", "queried")],
        );
        assert_eq!(
            feed.collect::<Vec<_>>(),
            vec![
                event(1, "db", "connected"),
                event(2, "app", "started"),
                event(2, "app", "ready"),
                event(2, "db", "queried"),
                event(5, "app", "served"),
            ]
        );
    }

    #[test]
    fn can_pause_then_resume_without_losing_events() {
        let mut feed = sorted_feed(
            vec![event(1, "app", "first"), event(4, "app", "last")],
            vec![event(2, "db", "second"), event(3, "db", "third")],
        );
        assert_eq!(feed.next(), Some(event(1, "app", "first")));
        assert_eq!(
            feed.by_ref().take(2).collect::<Vec<_>>(),
            vec![event(2, "db", "second"), event(3, "db", "third"),]
        );
        assert_eq!(feed.next(), Some(event(4, "app", "last")));
        assert_eq!(feed.next(), None);
        assert_eq!(feed.next(), None);
    }

    #[test]
    fn empty_and_single_input_feeds_finish_cleanly() {
        assert_eq!(sorted_feed(vec![], vec![]).next(), None);
        assert_eq!(
            sorted_feed(vec![], vec![event(7, "db", "only")]).collect::<Vec<_>>(),
            vec![event(7, "db", "only")]
        );
        assert_eq!(
            sorted_feed(vec![event(8, "app", "only")], vec![]).collect::<Vec<_>>(),
            vec![event(8, "app", "only")]
        );
    }
}
