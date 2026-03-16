import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Container, Navbar, Nav } from 'react-bootstrap';
import 'bootstrap/dist/css/bootstrap.min.css';
import './App.css';

import HomePage from './components/HomePage';
import BookingForm from './components/BookingForm';
import AvailabilitySearch from './components/AvailabilitySearch';
import VoiceBooking from './components/VoiceBooking';
import DocumentPreview from './components/DocumentPreview';

function App() {
  return (
    <Router>
      <div className="App">
        <Navbar bg="dark" variant="dark" expand="lg">
          <Container>
            <Navbar.Brand href="/">Medical Booking Platform</Navbar.Brand>
            <Nav className="me-auto">
              <Nav.Link href="/">Home</Nav.Link>
              <Nav.Link href="/book">Book Appointment</Nav.Link>
              <Nav.Link href="/search">Search Availability</Nav.Link>
              <Nav.Link href="/voice">Voice Booking</Nav.Link>
              <Nav.Link href="/documents">Documents</Nav.Link>
            </Nav>
          </Container>
        </Navbar>

        <Container className="mt-4">
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/book" element={<BookingForm />} />
            <Route path="/search" element={<AvailabilitySearch />} />
            <Route path="/voice" element={<VoiceBooking />} />
            <Route path="/documents" element={<DocumentPreview />} />
          </Routes>
        </Container>
      </div>
    </Router>
  );
}

export default App;