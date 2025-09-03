-- Drop trigger and function
DROP TRIGGER IF EXISTS update_users_updated_at ON public.users;
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop index
DROP INDEX IF EXISTS idx_users_role;

-- Remove columns from users table
ALTER TABLE public.users 
DROP COLUMN IF EXISTS password_hash,
DROP COLUMN IF EXISTS role,
DROP COLUMN IF EXISTS created_at,
DROP COLUMN IF EXISTS updated_at;
