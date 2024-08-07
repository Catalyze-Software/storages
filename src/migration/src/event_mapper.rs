use catalyze_shared::{
    event::Event,
    event_with_attendees::EventWithAttendees,
    general_structs::{
        members::Members, metadata::Metadata, privacy::Privacy, references::References,
    },
};

pub fn map_event(event: Event, attendees: Members) -> EventWithAttendees {
    EventWithAttendees {
        metadata: Metadata {
            name: event.name,
            description: event.description,
            banner_image: event.banner_image,
            image: event.image,
            location: event.location,
            website: event.website,
        },
        created_by: event.created_by,
        created_on: event.created_on,
        is_deleted: event.is_deleted,
        attendees,
        privacy: Privacy {
            privacy_type: event.privacy,
            privacy_gated_type_amount: None,
        },
        owner: event.owner,
        updated_on: event.updated_on,
        references: References {
            notification_id: None,
            tags: event.tags,
        },
        dates: vec![event.date],
        group_id: Some(event.group_id),
        is_canceled: if event.is_canceled.0 {
            Some(event.is_canceled.1)
        } else {
            None
        },
    }
}
