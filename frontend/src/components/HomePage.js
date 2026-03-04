import React from 'react';
import { Jumbotron, Button, Card, Row, Col } from 'react-bootstrap';

const HomePage = () => {
  return (
    <>
      <Jumbotron className="text-center py-5 mb-4">
        <h1>Welcome to Medical Booking Platform</h1>
        <p className="lead">
          Book appointments with healthcare professionals quickly and easily.
          Search availability by symptom, condition, or specialty.
        </p>
        <Button variant="primary" href="/book">Book Appointment</Button>
      </Jumbotron>

      <Row>
        <Col md={4}>
          <Card>
            <Card.Body>
              <Card.Title>Text Booking</Card.Title>
              <Card.Text>
                Fill out our easy-to-use form to book your appointment.
                Select from available time slots and healthcare providers.
              </Card.Text>
              <Button variant="outline-primary" href="/book">Book Now</Button>
            </Card.Body>
          </Card>
        </Col>
        <Col md={4}>
          <Card>
            <Card.Body>
              <Card.Title>Voice Booking</Card.Title>
              <Card.Text>
                Use your voice to book appointments hands-free.
                Our speech recognition technology makes booking effortless.
              </Card.Text>
              <Button variant="outline-primary" href="/voice">Voice Booking</Button>
            </Card.Body>
          </Card>
        </Col>
        <Col md={4}>
          <Card>
            <Card.Body>
              <Card.Title>Search Availability</Card.Title>
              <Card.Text>
                Find available appointments by symptom, condition, or specialty.
                See real-time availability from multiple healthcare providers.
              </Card.Text>
              <Button variant="outline-primary" href="/search">Search</Button>
            </Card.Body>
          </Card>
        </Col>
      </Row>
    </>
  );
};

export default HomePage;