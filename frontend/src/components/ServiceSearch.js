import React, { useState, useEffect } from 'react';
import { Form, Button, Card, Spinner, Alert, Table, Badge } from 'react-bootstrap';

const ServiceSearch = () => {
  const [selectedSymptoms, setSelectedSymptoms] = useState([]);
  const [allSymptoms, setAllSymptoms] = useState([]);
  const [symptomSearch, setSymptomSearch] = useState('');
  const [services, setServices] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  // Load all symptoms on mount
  useEffect(() => {
    const loadSymptoms = async () => {
      try {
        const response = await fetch('/api/symptoms');
        const data = await response.json();
        setAllSymptoms(data);
      } catch (err) {
        console.error('Error loading symptoms:', err);
      }
    };
    loadSymptoms();
  }, []);

  const handleSearch = async (e) => {
    e.preventDefault();
    setLoading(true);
    setError('');
    
    try {
      const response = await fetch('/api/services/by-symptoms', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          symptoms: selectedSymptoms,
        }),
      });
      const data = await response.json();
      
      if (data && data.length > 0) {
        setServices(data);
      } else {
        setError('No services found matching your symptoms.');
      }
    } catch (err) {
      setError('Error searching for services. Please try again.');
      console.error('Search error:', err);
    } finally {
      setLoading(false);
    }
  };

  const toggleSymptom = (symptomName) => {
    setSelectedSymptoms(prev => 
      prev.includes(symptomName) 
        ? prev.filter(s => s !== symptomName)
        : [...prev, symptomName]
    );
  };

  return (
    <Card>
      <Card.Header>
        <h2>Find Services by Symptoms</h2>
      </Card.Header>
      <Card.Body>
        <Form onSubmit={handleSearch}>
          <Form.Group className="mb-3">
            <Form.Label>Select Your Symptoms</Form.Label>
            <Form.Control 
              type="text" 
              value={symptomSearch}
              onChange={(e) => setSymptomSearch(e.target.value)}
              placeholder="Type to filter symptoms..."
              disabled={loading}
            />
            <div style={{ 
              maxHeight: '200px', 
              overflowY: 'auto', 
              border: '1px solid #dee2e6', 
              borderRadius: '0.25rem',
              marginTop: '0.5rem',
              padding: '0.5rem'
            }}>
              {allSymptoms
                .filter(s => s.name.toLowerCase().includes(symptomSearch.toLowerCase()))
                .map((symptom) => (
                  <Badge 
                    key={symptom.id}
                    bg={selectedSymptoms.includes(symptom.name) ? 'success' : 'secondary'}
                    pill
                    style={{ margin: '0.25rem', cursor: 'pointer', fontSize: '0.9rem' }}
                    onClick={() => toggleSymptom(symptom.name)}
                  >
                    {symptom.name} {selectedSymptoms.includes(symptom.name) ? '✓' : ''}
                  </Badge>
                ))
              }
            </div>
            {selectedSymptoms.length > 0 && (
              <div style={{ marginTop: '0.5rem', fontSize: '0.85rem', color: '#6c757d' }}>
                Selected: {selectedSymptoms.join(', ')}
              </div>
            )}
          </Form.Group>

          <Button type="submit" variant="primary" disabled={loading || selectedSymptoms.length === 0}>
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
              'Find Services'
            )}
          </Button>
        </Form>

        {error && (
          <Alert variant="danger" className="mt-3">
            {error}
          </Alert>
        )}

        {services.length > 0 && (
          <div className="mt-4">
            <h3>Recommended Services ({services.length})</h3>
            <Table striped bordered hover>
              <thead>
                <tr>
                  <th>Service</th>
                  <th>Duration</th>
                  <th>Price</th>
                  <th>Category</th>
                  <th>Addresses</th>
                </tr>
              </thead>
              <tbody>
                {services.map((service) => (
                  <tr key={service.id}>
                    <td><strong>{service.name}</strong></td>
                    <td>{service.duration_minutes} min</td>
                    <td>{service.price ? `$${service.price}` : 'N/A'}</td>
                    <td>{service.category || 'General'}</td>
                    <td>
                      {service.symptoms.map((s, idx) => (
                        <Badge bg="info" key={idx} style={{ margin: '0.1rem' }}>{s}</Badge>
                      ))}
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

export default ServiceSearch;
