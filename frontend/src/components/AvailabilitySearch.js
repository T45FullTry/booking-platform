import React, { useState } from 'react';
import { Form, Button, Table, Alert, Card, Spinner, Modal } from 'react-bootstrap';

const AvailabilitySearch = () => {
  const [symptom, setSymptom] = useState('');
  const [condition, setCondition] = useState('');
  const [specialty, setSpecialty] = useState('');
  const [patientId, setPatientId] = useState(''); // Assuming patient ID is available
  const [searchResults, setSearchResults] = useState([]);
  const [availabilitySlots, setAvailabilitySlots] = useState([]);
  const [selectedClinician, setSelectedClinician] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [showBookingModal, setShowBookingModal] = useState(false);
  const [selectedSlot, setSelectedSlot] = useState(null);

  const handleSearch = async (e) => {
    e.preventDefault();
    setLoading(true);
    setError('');
    
    try {
      const response = await fetch(`/api/search-clinicians?symptom=${encodeURIComponent(symptom)}&condition=${encodeURIComponent(condition)}&specialty=${encodeURIComponent(specialty)}`);
      const data = await response.json();
      
      if (data.results) {
        setSearchResults(data.results);
      } else {
        setError('No clinicians found matching your criteria.');
      }
    } catch (err) {
      setError('Error searching for clinicians. Please try again.');
      console.error('Search error:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleViewAvailability = async (clinicianId) => {
    setLoading(true);
    setError('');
    
    try {
      // For demo purposes, using today's date
      const today = new Date().toISOString().split('T')[0];
      const response = await fetch(`/api/availability?clinician_id=${clinicianId}&date=${today}&patient_id=${patientId}`);
      const data = await response.json();
      
      if (data.slots) {
        setAvailabilitySlots(data.slots);
        const clinician = searchResults.find(c => c.id === clinicianId);
        setSelectedClinician({...clinician, date: today});
      } else {
        setError('No availability found for this clinician.');
      }
    } catch (err) {
      setError('Error fetching availability. Please try again.');
      console.error('Availability error:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleBookAppointment = async (slotId) => {
    setSelectedSlot(slotId);
    setShowBookingModal(true);
  };

  const confirmBooking = async () => {
    setLoading(true);
    setError('');
    
    try {
      const response = await fetch('/api/bookings', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          patient_id: patientId,
          clinician_id: selectedClinician.id,
          service_id: '00000000-0000-0000-0000-000000000000', // Default service ID
          availability_slot_id: selectedSlot,
          booking_date: selectedClinician.date,
          booking_time: '09:00:00', // This would come from the slot in a real implementation
          duration_minutes: 30,
          symptoms_reported: [symptom],
          consultation_reason: `Patient presenting with ${symptom}`
        }),
      });
      
      if (response.ok) {
        setShowBookingModal(false);
        // Refresh availability to show updated slots
        handleViewAvailability(selectedClinician.id);
        alert('Appointment booked successfully!');
      } else {
        const errorData = await response.json();
        setError(`Booking failed: ${errorData}`);
      }
    } catch (err) {
      setError('Error booking appointment. Please try again.');
      console.error('Booking error:', err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card>
      <Card.Header>
        <h2>Search Clinician Availability</h2>
      </Card.Header>
      <Card.Body>
        <Form onSubmit={handleSearch}>
          <Form.Group className="mb-3">
            <Form.Label>Patient ID</Form.Label>
            <Form.Control 
              type="text" 
              value={patientId}
              onChange={(e) => setPatientId(e.target.value)}
              placeholder="Enter patient ID"
            />
          </Form.Group>
          
          <Form.Group className="mb-3">
            <Form.Label>Symptom</Form.Label>
            <Form.Control 
              type="text" 
              value={symptom}
              onChange={(e) => setSymptom(e.target.value)}
              placeholder="e.g., Headache, Fever, Cough"
            />
          </Form.Group>

          <Form.Group className="mb-3">
            <Form.Label>Medical Condition</Form.Label>
            <Form.Control 
              type="text" 
              value={condition}
              onChange={(e) => setCondition(e.target.value)}
              placeholder="e.g., Hypertension, Diabetes"
            />
          </Form.Group>

          <Form.Group className="mb-3">
            <Form.Label>Specialty</Form.Label>
            <Form.Select 
              value={specialty}
              onChange={(e) => setSpecialty(e.target.value)}
            >
              <option value="">Select a specialty</option>
              <option value="General Practice">General Practice</option>
              <option value="Cardiology">Cardiology</option>
              <option value="Dermatology">Dermatology</option>
              <option value="Neurology">Neurology</option>
              <option value="Pediatrics">Pediatrics</option>
              <option value="Psychiatry">Psychiatry</option>
              <option value="Orthopedics">Orthopedics</option>
            </Form.Select>
          </Form.Group>

          <Button type="submit" variant="primary" disabled={loading || !patientId}>
            {loading ? (
              <>
                <Spinner
                  as="span"
                  animation="border"
                  size="sm"
                  role="status"
                  aria-hidden="true"
                /> Searching...
              </>
            ) : (
              'Search Availability'
            )}
          </Button>
        </Form>

        {error && (
          <Alert variant="danger" className="mt-3">
            {error}
          </Alert>
        )}

        {searchResults.length > 0 && (
          <div className="mt-4">
            <h3>Available Clinicians ({searchResults.length})</h3>
            <Table striped bordered hover>
              <thead>
                <tr>
                  <th>Clinician</th>
                  <th>Specialty</th>
                  <th>Rating</th>
                  <th>Action</th>
                </tr>
              </thead>
              <tbody>
                {searchResults.map((clinician) => (
                  <tr key={clinician.id}>
                    <td>{clinician.name}</td>
                    <td>{clinician.specialty}</td>
                    <td>{clinician.rating} ★</td>
                    <td>
                      <Button 
                        variant="outline-primary" 
                        size="sm"
                        onClick={() => handleViewAvailability(clinician.id)}
                        disabled={!patientId}
                      >
                        View Availability
                      </Button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </Table>
          </div>
        )}

        {availabilitySlots.length > 0 && selectedClinician && (
          <div className="mt-4">
            <h3>Availability for {selectedClinician.name} on {selectedClinician.date}</h3>
            <Table striped bordered hover>
              <thead>
                <tr>
                  <th>Time Slot</th>
                  <th>Status</th>
                  <th>Action</th>
                </tr>
              </thead>
              <tbody>
                {availabilitySlots.map((slot) => (
                  <tr key={slot.id}>
                    <td>{slot.start_time} - {slot.end_time}</td>
                    <td>{slot.available ? 'Available' : 'Reserved'}</td>
                    <td>
                      <Button 
                        variant="success" 
                        size="sm"
                        onClick={() => handleBookAppointment(slot.id)}
                        disabled={!slot.available}
                      >
                        Book
                      </Button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </Table>
          </div>
        )}

        {/* Booking Confirmation Modal */}
        <Modal show={showBookingModal} onHide={() => setShowBookingModal(false)}>
          <Modal.Header closeButton>
            <Modal.Title>Confirm Booking</Modal.Title>
          </Modal.Header>
          <Modal.Body>
            Are you sure you want to book this appointment with {selectedClinician?.name}?
          </Modal.Body>
          <Modal.Footer>
            <Button variant="secondary" onClick={() => setShowBookingModal(false)}>
              Cancel
            </Button>
            <Button variant="primary" onClick={confirmBooking} disabled={loading}>
              {loading ? 'Booking...' : 'Confirm Booking'}
            </Button>
          </Modal.Footer>
        </Modal>
      </Card.Body>
    </Card>
  );
};

export default AvailabilitySearch;