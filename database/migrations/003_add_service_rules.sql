-- Service Rules Schema
-- Adds support for service eligibility rules (age, gender, recurrence, etc.)

-- Service rules table
CREATE TABLE service_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_id UUID NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    rule_type VARCHAR(50) NOT NULL, -- age_min, age_max, gender_required, recurrence_interval, recurrence_unit, appointment_limit, prerequisite_service
    rule_value VARCHAR(255), -- numeric value for age, 'M'/'F' for gender, number for interval
    rule_value_numeric INTEGER, -- for numeric comparisons
    description TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Rule types:
-- age_min: Minimum age required (rule_value_numeric = years)
-- age_max: Maximum age allowed (rule_value_numeric = years)
-- gender_required: Gender requirement (rule_value = 'M', 'F', or 'ANY')
-- recurrence_interval: Time between appointments (rule_value_numeric = number)
-- recurrence_unit: Unit for recurrence (rule_value = 'days', 'weeks', 'months', 'years')
-- appointment_limit: Max appointments per period (rule_value_numeric = count)
-- prerequisite_service: Required service before this one (rule_value = service_id UUID)

-- Indexes for performance
CREATE INDEX idx_service_rules_service_id ON service_rules(service_id);
CREATE INDEX idx_service_rules_rule_type ON service_rules(rule_type);
CREATE INDEX idx_service_rules_is_active ON service_rules(is_active);

-- Trigger for service_rules timestamp update
CREATE TRIGGER update_service_rules_updated_at BEFORE UPDATE ON service_rules
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Sample service rules for common services
-- General Checkup: No age restrictions, available to all genders
INSERT INTO service_rules (service_id, rule_type, rule_value, rule_value_numeric, description)
SELECT id, 'gender_required', 'ANY', NULL, 'Available to all genders'
FROM services WHERE name = 'General Checkup';

-- Consultation: No age restrictions
INSERT INTO service_rules (service_id, rule_type, rule_value, rule_value_numeric, description)
SELECT id, 'gender_required', 'ANY', NULL, 'Available to all genders'
FROM services WHERE name = 'Consultation';

-- Example: Mammography (would be a separate service) - females only, age 40+
-- Example: Prostate Screening (would be a separate service) - males only, age 50+
-- Example: Vaccination booster - recurring every 12 months

-- View for checking service eligibility
CREATE OR REPLACE VIEW service_eligibility AS
SELECT 
    s.id AS service_id,
    s.name AS service_name,
    sr.rule_type,
    sr.rule_value,
    sr.rule_value_numeric,
    sr.description
FROM services s
LEFT JOIN service_rules sr ON s.id = sr.service_id AND sr.is_active = TRUE
ORDER BY s.id, sr.rule_type;

-- Function to check if a patient is eligible for a service
CREATE OR REPLACE FUNCTION check_service_eligibility(
    patient_id UUID,
    service_id UUID
) RETURNS TABLE (
    eligible BOOLEAN,
    reason TEXT,
    failed_rules TEXT[]
) AS $$
DECLARE
    patient_dob DATE;
    patient_gender VARCHAR;
    age INTEGER;
    failed_rules_arr TEXT[] := ARRAY[]::TEXT[];
    rule RECORD;
BEGIN
    -- Get patient info
    SELECT date_of_birth, gender INTO patient_dob, patient_gender
    FROM patients WHERE id = patient_id;
    
    IF patient_dob IS NULL THEN
        RETURN QUERY SELECT FALSE, 'Patient not found', ARRAY['INVALID_PATIENT'];
        RETURN;
    END IF;
    
    -- Calculate age
    age := EXTRACT(YEAR FROM AGE(CURRENT_DATE, patient_dob))::INTEGER;
    
    -- Check all active rules for this service
    FOR rule IN 
        SELECT rule_type, rule_value, rule_value_numeric, description
        FROM service_rules
        WHERE service_id = check_service_eligibility.service_id
        AND is_active = TRUE
    LOOP
        IF rule.rule_type = 'age_min' AND rule.rule_value_numeric IS NOT NULL THEN
            IF age < rule.rule_value_numeric THEN
                failed_rules_arr := failed_rules_arr || 
                    format('AGE_MIN(%s)', rule.rule_value_numeric);
            END IF;
        ELSIF rule.rule_type = 'age_max' AND rule.rule_value_numeric IS NOT NULL THEN
            IF age > rule.rule_value_numeric THEN
                failed_rules_arr := failed_rules_arr || 
                    format('AGE_MAX(%s)', rule.rule_value_numeric);
            END IF;
        ELSIF rule.rule_type = 'gender_required' AND rule.rule_value IS NOT NULL THEN
            IF rule.rule_value != 'ANY' THEN
                IF (rule.rule_value = 'M' AND patient_gender != 'Male') OR
                   (rule.rule_value = 'F' AND patient_gender != 'Female') THEN
                    failed_rules_arr := failed_rules_arr || 
                        format('GENDER(%s)', rule.rule_value);
                END IF;
            END IF;
        END IF;
    END LOOP;
    
    IF array_length(failed_rules_arr, 1) > 0 THEN
        RETURN QUERY SELECT FALSE, 'Patient does not meet service requirements', failed_rules_arr;
    ELSE
        RETURN QUERY SELECT TRUE, 'Patient eligible for service', failed_rules_arr;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Function to get available services for a patient
CREATE OR REPLACE FUNCTION get_available_services_for_patient(
    patient_id UUID
) RETURNS TABLE (
    service_id UUID,
    service_name VARCHAR,
    description TEXT,
    duration_minutes INTEGER,
    price DECIMAL,
    category VARCHAR,
    eligibility_status BOOLEAN,
    eligibility_reason TEXT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        s.id,
        s.name,
        s.description,
        s.duration_minutes,
        s.price,
        s.category,
        (SELECT eligible FROM check_service_eligibility(patient_id, s.id) LIMIT 1),
        (SELECT reason FROM check_service_eligibility(patient_id, s.id) LIMIT 1)
    FROM services s
    WHERE s.id IN (
        SELECT DISTINCT service_id FROM service_rules WHERE is_active = TRUE
        UNION ALL
        SELECT id FROM services WHERE id NOT IN (
            SELECT service_id FROM service_rules
        )
    )
    ORDER BY s.name;
END;
$$ LANGUAGE plpgsql;
