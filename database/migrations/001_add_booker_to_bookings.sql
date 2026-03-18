-- Migration: Add booker_id to bookings table
-- This allows booking appointments on behalf of another person

-- Add booker_id column (references patients table)
ALTER TABLE bookings 
ADD COLUMN booker_id UUID NOT NULL REFERENCES patients(id);

-- Add comment to document the purpose
COMMENT ON COLUMN bookings.booker_id IS 'The person making the booking (may be different from patient_id)';
COMMENT ON COLUMN bookings.patient_id IS 'The person the appointment is for (appointee)';

-- Create index for performance
CREATE INDEX IF NOT EXISTS idx_bookings_booker_id ON bookings(booker_id);

-- Update existing bookings: set booker_id = patient_id (existing bookings were self-booked)
UPDATE bookings SET booker_id = patient_id WHERE booker_id IS NULL;

-- Add trigger to update updated_at timestamp (already exists via update_bookings_updated_at)
-- No additional trigger needed as the existing trigger handles all updates
