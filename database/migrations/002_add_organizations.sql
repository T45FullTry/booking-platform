-- Organizations and Organization Types Schema
-- Adds support for tracking organizations (schools, insurance, charities, government, businesses, etc.)

-- Organization types table (e.g., school, insurance, charity, government, business, healthcare, utility)
CREATE TABLE organization_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Organizations table
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    organization_type_id UUID NOT NULL REFERENCES organization_types(id),
    registration_number VARCHAR(100), -- Business registration, charity number, etc.
    tax_id VARCHAR(100), -- Tax identification number
    website VARCHAR(255),
    email VARCHAR(255),
    phone VARCHAR(20),
    fax VARCHAR(20),
    address TEXT,
    city VARCHAR(100),
    state_province VARCHAR(100),
    postal_code VARCHAR(20),
    country VARCHAR(100),
    contact_person_name VARCHAR(100),
    contact_person_email VARCHAR(255),
    contact_person_phone VARCHAR(20),
    notes TEXT,
    status VARCHAR(20) DEFAULT 'active', -- active, inactive, suspended
    metadata JSONB, -- Additional organization-specific data
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Patient-Employment table (links patients to organizations as employers)
CREATE TABLE patient_employments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES patients(id),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    job_title VARCHAR(100),
    department VARCHAR(100),
    employee_id VARCHAR(50), -- Employee reference number
    start_date DATE,
    end_date DATE,
    is_current BOOLEAN DEFAULT TRUE,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Clinician-Affiliation table (links clinicians to organizations as workplaces)
CREATE TABLE clinician_affiliations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    clinician_id UUID NOT NULL REFERENCES clinicians(id),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    affiliation_type VARCHAR(50), -- employed, contracted, visiting, privileges
    department VARCHAR(100),
    start_date DATE,
    end_date DATE,
    is_primary BOOLEAN DEFAULT FALSE,
    is_current BOOLEAN DEFAULT TRUE,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Booking-Insurance table (links bookings to insurance organizations)
CREATE TABLE booking_insurance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    booking_id UUID NOT NULL REFERENCES bookings(id),
    organization_id UUID NOT NULL REFERENCES organizations(id), -- Insurance provider
    policy_number VARCHAR(100),
    group_number VARCHAR(100),
    member_id VARCHAR(100),
    coverage_type VARCHAR(50), -- primary, secondary, supplemental
    authorization_required BOOLEAN DEFAULT FALSE,
    authorization_number VARCHAR(100),
    claim_status VARCHAR(20), -- pending, approved, denied, partially_approved
    claim_amount DECIMAL(10,2),
    patient_responsibility DECIMAL(10,2),
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Document-Issuer table (links documents to issuing organizations)
CREATE TABLE document_issuers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL REFERENCES documents(id),
    organization_id UUID NOT NULL REFERENCES organizations(id), -- Issuing organization (lab, hospital, etc.)
    issuer_name VARCHAR(200), -- Specific department or person
    issue_date DATE,
    reference_number VARCHAR(100), -- Lab reference, report number, etc.
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insert default organization types
INSERT INTO organization_types (name, description) VALUES
('school', 'Educational institutions including schools, colleges, universities'),
('insurance', 'Insurance providers including health, life, and disability insurance'),
('charity', 'Non-profit and charitable organizations'),
('government', 'Government agencies and departments'),
('business', 'Commercial companies and enterprises'),
('healthcare', 'Healthcare facilities including hospitals, clinics, medical centers'),
('utility', 'Service providers including electric, water, internet, telecommunications'),
('laboratory', 'Medical and diagnostic laboratories'),
('pharmacy', 'Pharmacies and pharmaceutical dispensers'),
('research', 'Research institutions and clinical trial organizations'),
('other', 'Organizations that do not fit into other categories');

-- Indexes for performance
CREATE INDEX idx_organizations_type_id ON organizations(organization_type_id);
CREATE INDEX idx_organizations_name ON organizations(name);
CREATE INDEX idx_organizations_status ON organizations(status);
CREATE INDEX idx_patient_employments_patient_id ON patient_employments(patient_id);
CREATE INDEX idx_patient_employments_organization_id ON patient_employments(organization_id);
CREATE INDEX idx_clinician_affiliations_clinician_id ON clinician_affiliations(clinician_id);
CREATE INDEX idx_clinician_affiliations_organization_id ON clinician_affiliations(organization_id);
CREATE INDEX idx_booking_insurance_booking_id ON booking_insurance(booking_id);
CREATE INDEX idx_booking_insurance_organization_id ON booking_insurance(organization_id);
CREATE INDEX idx_document_issuers_document_id ON document_issuers(document_id);
CREATE INDEX idx_document_issuers_organization_id ON document_issuers(organization_id);

-- Trigger for organizations timestamp update
CREATE TRIGGER update_organizations_updated_at BEFORE UPDATE ON organizations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Trigger for patient_employments timestamp update
CREATE TRIGGER update_patient_employments_updated_at BEFORE UPDATE ON patient_employments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Trigger for clinician_affiliations timestamp update
CREATE TRIGGER update_clinician_affiliations_updated_at BEFORE UPDATE ON clinician_affiliations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Trigger for booking_insurance timestamp update
CREATE TRIGGER update_booking_insurance_updated_at BEFORE UPDATE ON booking_insurance
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
