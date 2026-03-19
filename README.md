# Medical Booking Platform

A comprehensive medical appointment booking platform with voice and text interfaces, built with Rust backend and React frontend.

## Features

### Frontend (React + Bootstrap)
- **Voice Booking**: Speech recognition for hands-free appointment booking
- **Text Booking**: Traditional form-based booking interface
- **Book on Behalf**: Book appointments for another person (family member, dependent, etc.) while including the booker in the appointment
- **Availability Search**: Search clinicians by symptom, condition, or specialty
- **Document Preview**: View medical documents by category (sick notes, referrals, prescriptions, lab results)
- **Responsive Design**: Mobile-friendly interface with Bootstrap
- **Modern UI**: Clean, professional healthcare interface

### Backend (Rust)
- **Main API Service**: Actix-web based REST API for bookings and patient management
- **Document API**: CRUD operations for medical documents with streaming support
- **Microservices**: Specialized services for complex operations
- **Database Integration**: PostgreSQL with comprehensive schema
- **High Performance**: Rust's speed and memory safety

### Microservices
- **Clinician Search Service**: Specialized service for finding clinicians by symptoms/conditions
- **Document Service**: Dedicated service for document streaming, upload, and management

### Database
- **PostgreSQL**: Robust relational database with comprehensive medical entities
- **Entities**: Patients, Clinicians, Services, Symptoms, Conditions, Bookings (with booker_id for proxy booking), Availability Slots, Consultations, Prescriptions, Documents, Organizations, Organization Types, Patient Employments, Clinician Affiliations, Booking Insurance, Document Issuers

## Project Structure

```
booking-platform/
├── backend/                 # Main Rust API service
│   ├── src/
│   │   ├── main.rs          # Application entry point
│   │   ├── models.rs        # Data structures
│   │   ├── handlers.rs      # Request handlers
│   │   └── db.rs           # Database operations
│   └── Cargo.toml          # Dependencies
├── frontend/               # React frontend
│   ├── src/
│   │   ├── components/      # React components
│   │   │   ├── DocumentPreview.js  # Document preview UI
│   │   │   ├── BookingForm.js     # Booking form
│   │   │   ├── AvailabilitySearch.js # Search UI
│   │   │   └── VoiceBooking.js    # Voice booking
│   │   ├── App.js          # Main application
│   │   └── index.js         # Entry point
│   └── package.json        # Dependencies
├── microservices/          # Specialized services
│   ├── clinician-search/   # Clinician search microservice
│   └── document-service/   # Document streaming & management
├── database/               # Database schema and migrations
│   └── schema.sql          # Complete database schema
└── docs/                   # Documentation
```

## Database Schema

The platform includes comprehensive database entities:

- **Patients**: Personal and medical information
- **Clinicians**: Healthcare providers with specialties and ratings
- **Services**: Types of medical services offered
- **Symptoms**: Reported symptoms for matching with conditions
- **Conditions**: Medical conditions with ICD codes
- **Availability Slots**: Time slots for appointments
- **Bookings**: Appointment bookings with status tracking
- **Consultations**: Post-appointment records
- **Prescriptions**: Medication prescriptions
- **Documents**: Medical documents with categories (sick notes, referrals, prescriptions, lab results, etc.), binary content storage, and patient visibility controls

## API Endpoints

### Main Service (Port 8080)
```
POST   /api/patients              # Create patient
POST   /api/bookings              # Create booking
GET    /api/bookings/{id}         # Get booking details
POST   /api/bookings/cancel       # Cancel booking
GET    /api/availability          # Get availability slots
GET    /api/clinicians/search     # Search clinicians (proxies to microservice)
GET    /api/clinicians/search-db  # Search clinicians (direct DB)
GET    /api/clinicians/{id}       # Get clinician details
POST   /api/documents             # Upload document
GET    /api/documents             # List documents (filter by category)
GET    /api/documents/{id}        # Get document details
PUT    /api/documents/{id}        # Update document
DELETE /api/documents/{id}        # Soft delete document
GET    /api/documents/{id}/stream # Stream document content
```

### Microservices (Port 8081)
```
GET    /api/search-clinicians     # Search clinicians by criteria
GET    /api/clinicians/{id}       # Get detailed clinician information
POST   /api/documents             # Upload document
GET    /api/documents             # List documents
GET    /api/documents/{id}        # Get document details
PUT    /api/documents/{id}        # Update document
DELETE /api/documents/{id}        # Delete document
GET    /api/documents/{id}/stream # Stream document content
```

### Organization Management (Port 8080)
```
GET    /api/organizations/types           # List all organization types (school, insurance, charity, etc.)
POST   /api/organizations                 # Create new organization
GET    /api/organizations                 # List organizations (filterable by type, status, location)
GET    /api/organizations/{id}            # Get organization details
PUT    /api/organizations/{id}            # Update organization
DELETE /api/organizations/{id}            # Soft delete organization (sets status to inactive)
POST   /api/patients/{id}/employments    # Record patient employment at organization
GET    /api/patients/{id}/employments    # Get patient's employment history
POST   /api/clinicians/{id}/affiliations # Record clinician affiliation with organization
GET    /api/clinicians/{id}/affiliations # Get clinician's affiliations
POST   /api/bookings/{id}/insurance      # Record insurance info for booking
GET    /api/bookings/{id}/insurance      # Get booking insurance info
POST   /api/documents/{id}/issuers       # Record document issuing organization
GET    /api/documents/{id}/issuers       # Get document issuers
```

## Setup Instructions

### Prerequisites
- Rust and Cargo
- Node.js and npm
- PostgreSQL database
- Environment variables in `.env` files

### Backend Setup
```bash
cd backend
cargo build
DATABASE_URL=postgresql://user:password@localhost/booking_platform cargo run
```

### Microservice Setup
```bash
# Clinician Search Service
cd microservices/clinician-search
cargo build
DATABASE_URL=postgresql://user:password@localhost/booking_platform cargo run

# Document Service (Port 8081)
cd microservices/document-service
cargo build
DATABASE_URL=postgresql://user:password@localhost/booking_platform cargo run
```

### Frontend Setup
```bash
cd frontend
npm install
npm start
# Navigate to /documents for document preview UI
```

### Database Setup
```bash
psql -U username -d booking_platform -f database/schema.sql
```

## Technology Stack

### Backend
- **Rust**: Systems programming language
- **Actix-web**: Web framework
- **SQLx**: Database toolkit
- **PostgreSQL**: Relational database
- **Base64**: Document content encoding

### Frontend
- **React**: JavaScript library for UI
- **Bootstrap**: CSS framework
- **React Bootstrap**: Bootstrap components for React
- **Web Speech API**: Voice recognition
- **File/Blob handling**: Document preview rendering

### Microservices
- **Rust**: For performance-critical services
- **Actix-web**: Consistent with main service

## Future Enhancements

- Telemedicine video consultations
- SMS and email notifications
- Insurance verification
- Analytics dashboard
- Multi-language support
- Document OCR and search
- E-prescription integration with pharmacies
- Patient portal for document access

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a pull request

## License

This project is licensed under the MIT License.