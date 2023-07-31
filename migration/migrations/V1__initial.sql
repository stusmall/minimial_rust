CREATE TABLE messages
(
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    message text NOT NULL
);

