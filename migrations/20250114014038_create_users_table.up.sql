CREATE TABLE public.users (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL
);
CREATE UNIQUE INDEX users_email_key ON public.users (email);