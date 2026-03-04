import React, { useState } from 'react';
import { Form, Button, Table, Alert, Card, Spinner } from 'react-bootstrap';

const AvailabilitySearch = () => {
  const [symptom, setSymptom] = useState('');
  const [condition, setCondition] = useState('');
  const [specialty, setSpecialty] = useState('');
  const [searchResults, setSearchResults] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

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
      setError('Error searching for availability. Please try again.');
      console.error('Search error:', err);
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

          <Button type="submit" variant="primary" disabled={loading}>
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
                  <th>Available Times</th>
                  <th>Rating</th>
                  <th>Action</th>
                </tr>
              </thead>
              <tbody>
                {searchResults.map((clinician) => (
                  <tr key={clinician.id}>
                    <td>{clinician.name}</td>
                    <td>{clinician.specialty}</td>
                    <td>
                      {clinician.available_times.slice(0, 3).map((time, idx) => (
                        <div key={idx}>{time}</div>
                      ))}
                      {clinician.available_times.length > 3 && (
                        <div>+{clinician.available_times.length - 3} more</div>
                      )}
                    </td>
                    <td>{clinician.rating} ★</td>
                    <td>
                      <Button 
                        variant="outline-primary" 
                        size="sm"
                        href={`/book?clinician=${clinician.id}`}
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
      </Card.Body>
    </Card>
  );
};

export default AvailabilitySearch;