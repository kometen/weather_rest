table! {
    readings {
        measurement_time_default -> Timestamptz,
        id -> Integer,
        data -> Jsonb,
    }
}

table! {
    latest_readings {
        measurement_time_default -> Timestamptz,
        id -> Integer,
        data -> Jsonb,
    }
}

table! {
    locations {
        publication_time -> Timestamptz,
        id -> Integer,
        name -> Text,
        latitude -> Numeric,
        longitude -> Numeric,
    }
}

table! {
    location_readings {
        measurement_time_default -> Timestamptz,
        id -> Integer,
        name -> Text,
        latitude -> Numeric,
        longitude -> Numeric,
        data -> Jsonb,
    }
}

table! {
    measurements_single_location_function {
        id -> Integer,
        name -> Text,
        latitude -> Text,
        longitude -> Text,
        measurement_time_default -> Timestamptz,
        measurements -> Jsonb,
    }
}
