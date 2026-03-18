import React, { useState, useEffect } from 'react';
import { Form, Button, Alert, Card } from 'react-bootstrap';
import SpeechRecognition, { useSpeechRecognition } from 'react-speech-recognition';

const VoiceBooking = () => {
  const [symptom, setSymptom] = useState('');
  const [condition, setCondition] = useState('');
  const [preferredDate, setPreferredDate] = useState('');
  const [searchResults, setSearchResults] = useState([]);
  const [bookingConfirmed, setBookingConfirmed] = useState(false);
  const [patientId, setPatientId] = useState(''); // The person the appointment is for (appointee)
  const [bookerId, setBookerId] = useState('');   // The person making the booking
  const [isBookingForSelf, setIsBookingForSelf] = useState(true);

  const {
    transcript,
    listening,
    resetTranscript,
    browserSupportsSpeechRecognition
  } = useSpeechRecognition();

  useEffect(() => {
    if (transcript) {
      // Simple parsing of voice input
      if (transcript.toLowerCase().includes('headache')) setSymptom('Headache');
      if (transcript.toLowerCase().includes('fever')) setSymptom('Fever');
      if (transcript.toLowerCase().includes('cough')) setSymptom('Cough');
      if (transcript.toLowerCase().includes('tomorrow')) setPreferredDate('tomorrow');
    }
  }, [transcript]);

  const startListening = () => {
    SpeechRecognition.startListening({ continuous: true });
  };

  const stopListening = () => {
    SpeechRecognition.stopListening();
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    
    // Search for availability based on symptoms
    try {
      const response = await fetch(`/api/search-clinicians?symptom=${symptom}&condition=${condition}`);
      const data = await response.json();
      setSearchResults(data.results);
    } catch (error) {
      console.error('Error searching availability:', error);
    }
  };

  const handleBooking = async (clinicianId) => {
    try {
      const bookingData = {
        patient_id: patientId || bookerId, // Default to booker if no patient specified
        booker_id: bookerId,
        clinician_id: clinicianId,
        service_id: '1', // Default service - in production, this should be selected
        booking_date: preferredDate,
        booking_time: '09:00', // Default time - in production, this should be selected
        duration_minutes: 30,
        symptoms_reported: symptom ? [symptom] : null,
      };
      
      const response = await fetch('/api/bookings', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(bookingData),
      });
      
      if (response.ok) {
        setBookingConfirmed(true);
      }
    } catch (error) {
      console.error('Error booking appointment:', error);
    }
  };

  if (!browserSupportsSpeechRecognition) {
    return (
      <Alert variant="danger">
        Browser doesn't support speech recognition.
      </Alert>
    );
  }

  return (
    <Card>
      <Card.Header>
        <h2>Voice Booking</h2>
      </Card.Header>
      <Card.Body>
        <Form onSubmit={handleSubmit}>
          <Form.Group className="mb-3">
            <Form.Label>Booking For:</Form.Label>
            <Form.Check 
              type="checkbox"
              label="Booking for myself"
              checked={isBookingForSelf}
              onChange={(e) => {
                setIsBookingForSelf(e.target.checked);
                if (e.target.checked) {
                  setPatientId(bookerId);
                } else {
                  setPatientId('');
                }
              }}
            />
          </Form.Group>

          <Form.Group className="mb-3">
            <Form.Label>Your Patient ID (Booker):</Form.Label>
            <Form.Control 
              type="text" 
              value={bookerId}
              onChange={(e) => {
                setBookerId(e.target.value);
                if (isBookingForSelf) {
                  setPatientId(e.target.value);
                }
              }}
              placeholder="Enter your patient ID"
              required
            />
            <Form.Text className="text-muted">
              This is the person making the booking
            </Form.Text>
          </Form.Group>

          {!isBookingForSelf && (
            <Form.Group className="mb-3">
              <Form.Label>Patient ID (Appointment For):</Form.Label>
              <Form.Control 
                type="text" 
                value={patientId}
                onChange={(e) => setPatientId(e.target.value)}
                placeholder="Enter patient ID for the appointment"
                required={!isBookingForSelf}
              />
              <Form.Text className="text-muted">
                This is the person the appointment is for
              </Form.Text>
            </Form.Group>
          )}

          <Form.Group className="mb-3">
            <Form.Label>Speak your symptoms or booking request:</Form.Label>
            <div className="d-flex gap-2">
              <Button 
                onClick={listening ? stopListening : startListening}
                variant={listening ? "danger" : "success"}
              >
                {listening ? 'Stop Listening' : 'Start Speaking'}
              </Button>
              <Button onClick={resetTranscript} variant="secondary">
                Reset
              </Button>
            </div>
            <Form.Text className="text-muted">
              {listening ? "Listening..." : "Click 'Start Speaking' and describe your symptoms"}
            </Form.Text>
          </Form.Group>

          <Form.Group className="mb-3">
            <Form.Label>Recognized Text:</Form.Label>
            <Form.Control 
              as="textarea" 
              rows={3} 
              value={transcript}
              readOnly
            />
          </Form.Group>

          <Form.Group className="mb-3">
            <Form.Label>Symptoms Detected:</Form.Label>
            <Form.Control 
              type="text" 
              value={symptom}
              onChange={(e) => setSymptom(e.target.value)}
              placeholder="Enter symptoms"
            />
          </Form.Group>

          <Form.Group className="mb-3">
            <Form.Label>Preferred Date:</Form.Label>
            <Form.Control 
              type="date" 
              value={preferredDate}
              onChange={(e) => setPreferredDate(e.target.value)}
            />
          </Form.Group>

          <Button type="submit" variant="primary">
            Search Availability
          </Button>
        </Form>

        {searchResults.length > 0 && (
          <div className="mt-4">
            <h3>Available Clinicians:</h3>
            {searchResults.map((clinician) => (
              <Card key={clinician.id} className="mb-2">
                <Card.Body>
                  <Card.Title>{clinician.name}</Card.Title>
                  <Card.Text>
                    Specialty: {clinician.specialty}<br/>
                    Available: {clinician.available_times.join(', ')}
                  </Card.Text>
                  <Button 
                    variant="success" 
                    onClick={() => handleBooking(clinician.id)}
                  >
                    Book with {clinician.name}
                  </Button>
                </Card.Body>
              </Card>
            ))}
          </div>
        )}

        {bookingConfirmed && (
          <Alert variant="success" className="mt-3">
            Your appointment has been booked successfully!
          </Alert>
        )}
      </Card.Body>
    </Card>
  );
};

export default VoiceBooking;