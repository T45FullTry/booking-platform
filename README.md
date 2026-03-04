# Medical Booking Platform

A comprehensive medical appointment booking platform with voice and text interfaces, built with Rust backend and React frontend.

## Features

### Frontend (React + Bootstrap)
- **Voice Booking**: Speech recognition for hands-free appointment booking
- **Text Booking**: Traditional form-based booking interface
- **Availability Search**: Search clinicians by symptom, condition, or specialty
- **Responsive Design**: Mobile-friendly interface with Bootstrap
- **Modern UI**: Clean, professional healthcare interface

### Backend (Rust)
- **Main API Service**: Actix-web based REST API for bookings and patient management
- **Microservices**: Specialized services for complex operations
- **Database Integration**: PostgreSQL with comprehensive schema
- **High Performance**: Rust's speed and memory safety

### Microservices
- **Clinician Search Service**: Specialized service for finding clinicians by symptoms/conditions

### Database
- **PostgreSQL**: Robust relational database with comprehensive medical entities
- **Entities**: Patients, Clinicians, Services, Symptoms, Conditions, Bookings, Availability Slots, Consultations

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
│   │   ├── App.js          # Main application
│   │   └── index.js         # Entry point
│   └── package.json        # Dependencies
├── microservices/          # Specialized services
│   └── clinician-search/   # Clinician search microservice
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
```

### Microservices (Port 8081)
```
GET    /api/search-clinicians     # Search clinicians by criteria
GET    /api/clinicians/{id}       # Get detailed clinician information
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
cd microservices/clinician-search
cargo build
DATABASE_URL=postgresql://user:password@localhost/booking_platform cargo run
```

### Frontend Setup
```bash
cd frontend
npm install
npm start
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

### Frontend
- **React**: JavaScript library for UI
- **Bootstrap**: CSS framework
- **React Bootstrap**: Bootstrap components for React
- **Web Speech API**: Voice recognition

### Microservices
- **Rust**: For performance-critical services
- **Actix-web**: Consistent with main service

## Future Enhancements

- Telemedicine video consultations
- SMS and email notifications
- Insurance verification
- Medical record integration
- Analytics dashboard
- Multi-language support

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a pull request

## License

This project is licensed under the MIT License.