-- Medical Booking Platform Database Schema

-- Patients table
CREATE TABLE patients (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    date_of_birth DATE NOT NULL,
    gender VARCHAR(20),
    phone VARCHAR(20),
    email VARCHAR(255) UNIQUE,
    address TEXT,
    emergency_contact_name VARCHAR(100),
    emergency_contact_phone VARCHAR(20),
    medical_history TEXT,
    allergies TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Clinicians table
CREATE TABLE clinicians (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    title VARCHAR(50),
    specialty VARCHAR(100) NOT NULL,
    license_number VARCHAR(100) UNIQUE,
    education TEXT,
    experience_years INTEGER,
    bio TEXT,
    phone VARCHAR(20),
    email VARCHAR(255),
    office_location TEXT,
    languages TEXT[], -- Array of languages spoken
    rating NUMERIC(3,2) DEFAULT 0.00,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Services table
CREATE TABLE services (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    description TEXT,
    duration_minutes INTEGER NOT NULL,
    price DECIMAL(10,2),
    category VARCHAR(100),
    specialty_required VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Symptoms table
CREATE TABLE symptoms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    body_system VARCHAR(50), -- e.g., cardiovascular, respiratory, etc.
    severity_level INTEGER, -- 1-10 scale
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Conditions table
CREATE TABLE conditions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL UNIQUE,
    description TEXT,
    icd_code VARCHAR(20), -- International Classification of Diseases code
    chronic BOOLEAN DEFAULT FALSE,
    contagious BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Symptom-Condition mapping (many-to-many)
CREATE TABLE symptom_conditions (
    symptom_id UUID REFERENCES symptoms(id),
    condition_id UUID REFERENCES conditions(id),
    PRIMARY KEY (symptom_id, condition_id)
);

-- Availability slots table
CREATE TABLE availability_slots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    clinician_id UUID NOT NULL REFERENCES clinicians(id),
    date DATE NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    is_recurring BOOLEAN DEFAULT FALSE,
    recurrence_pattern VARCHAR(50), -- daily, weekly, monthly
    recurrence_end_date DATE,
    max_patients INTEGER DEFAULT 1,
    booked_patients INTEGER DEFAULT 0,
    status VARCHAR(20) DEFAULT 'available', -- available, booked, blocked
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Bookings table
CREATE TABLE bookings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES patients(id),
    clinician_id UUID NOT NULL REFERENCES clinicians(id),
    service_id UUID NOT NULL REFERENCES services(id),
    availability_slot_id UUID REFERENCES availability_slots(id),
    booking_date DATE NOT NULL,
    booking_time TIME NOT NULL,
    duration_minutes INTEGER NOT NULL,
    status VARCHAR(20) DEFAULT 'confirmed', -- confirmed, cancelled, completed, no-show
    consultation_reason TEXT,
    symptoms_reported TEXT[],
    urgency_level INTEGER, -- 1-5 scale
    notes TEXT,
    confirmation_code VARCHAR(50) UNIQUE,
    payment_status VARCHAR(20) DEFAULT 'pending', -- pending, paid, refunded
    amount_paid DECIMAL(10,2),
    cancellation_reason TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Consultations table
CREATE TABLE consultations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    booking_id UUID NOT NULL REFERENCES bookings(id),
    patient_id UUID NOT NULL REFERENCES patients(id),
    clinician_id UUID NOT NULL REFERENCES clinicians(id),
    consultation_date DATE NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    diagnosis TEXT,
    treatment_plan TEXT,
    prescription_given BOOLEAN DEFAULT FALSE,
    follow_up_required BOOLEAN DEFAULT FALSE,
    follow_up_days INTEGER,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Prescription table
CREATE TABLE prescriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consultation_id UUID NOT NULL REFERENCES consultations(id),
    patient_id UUID NOT NULL REFERENCES patients(id),
    clinician_id UUID NOT NULL REFERENCES clinicians(id),
    medication_name VARCHAR(200) NOT NULL,
    dosage VARCHAR(100),
    frequency VARCHAR(100),
    duration_days INTEGER,
    instructions TEXT,
    prescribed_date DATE NOT NULL,
    status VARCHAR(20) DEFAULT 'active', -- active, completed, discontinued
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_patients_email ON patients(email);
CREATE INDEX idx_clinicians_specialty ON clinicians(specialty);
CREATE INDEX idx_bookings_patient_id ON bookings(patient_id);
CREATE INDEX idx_bookings_clinician_id ON bookings(clinician_id);
CREATE INDEX idx_bookings_date ON bookings(booking_date);
CREATE INDEX idx_availability_clinician_date ON availability_slots(clinician_id, date);
CREATE INDEX idx_consultations_booking_id ON consultations(booking_id);
CREATE INDEX idx_symptoms_name ON symptoms(name);
CREATE INDEX idx_conditions_name ON conditions(name);

-- Trigger functions for automatic timestamp updates
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for automatic timestamp updates
CREATE TRIGGER update_patients_updated_at BEFORE UPDATE ON patients
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_clinicians_updated_at BEFORE UPDATE ON clinicians
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_availability_slots_updated_at BEFORE UPDATE ON availability_slots
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bookings_updated_at BEFORE UPDATE ON bookings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_consultations_updated_at BEFORE UPDATE ON consultations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_prescriptions_updated_at BEFORE UPDATE ON prescriptions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Sample data for initial setup
-- Insert some common symptoms
INSERT INTO symptoms (name, description, body_system, severity_level) VALUES
('Headache', 'Pain in the head or upper neck', 'Nervous', 3),
('Fever', 'Elevated body temperature', 'General', 4),
('Cough', 'Forceful expulsion of air from the lungs', 'Respiratory', 2),
('Fatigue', 'Feeling of tiredness or exhaustion', 'General', 3),
('Nausea', 'Feeling of sickness with an urge to vomit', 'Digestive', 3);

-- Insert some common conditions
INSERT INTO conditions (name, description, icd_code, chronic) VALUES
('Hypertension', 'High blood pressure', 'I10', TRUE),
('Diabetes Mellitus', 'Chronic disease affecting blood sugar', 'E11', TRUE),
('Common Cold', 'Viral infection of the nose and throat', 'J00', FALSE),
('Anxiety Disorder', 'Feelings of worry and fear', 'F41.9', TRUE),
('Depression', 'Persistent feeling of sadness', 'F32.9', TRUE);

-- Map symptoms to conditions
INSERT INTO symptom_conditions (symptom_id, condition_id)
SELECT s.id, c.id FROM symptoms s, conditions c
WHERE s.name = 'Headache' AND c.name = 'Hypertension';

INSERT INTO symptom_conditions (symptom_id, condition_id)
SELECT s.id, c.id FROM symptoms s, conditions c
WHERE s.name = 'Fatigue' AND c.name = 'Diabetes Mellitus';

-- Insert some services
INSERT INTO services (name, description, duration_minutes, price, category) VALUES
('General Checkup', 'Routine health examination', 30, 100.00, 'Preventive'),
('Consultation', 'Discussion with healthcare provider', 15, 75.00, 'Consultation'),
('Follow-up Visit', 'Post-treatment check', 20, 50.00, 'Follow-up');

-- Insert sample clinicians
INSERT INTO clinicians (first_name, last_name, title, specialty, license_number, experience_years, rating) VALUES
('Sarah', 'Johnson', 'Dr.', 'General Practice', 'MD12345', 15, 4.8),
('Michael', 'Chen', 'Dr.', 'Cardiology', 'MD67890', 12, 4.9),
('Emily', 'Rodriguez', 'Dr.', 'Neurology', 'MD54321', 10, 4.7);