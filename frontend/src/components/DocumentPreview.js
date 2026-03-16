import React, { useState, useEffect } from 'react';
import { Container, Card, Button, Form, Alert, Spinner, Badge, ListGroup, Modal } from 'react-bootstrap';

const API_BASE_URL = 'http://localhost:8080/api';

const DOCUMENT_CATEGORIES = [
  'sick_note',
  'referral',
  'prescription',
  'lab_result',
  'medical_report',
  'discharge_summary',
  'other'
];

function DocumentPreview() {
  const [documents, setDocuments] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [selectedCategory, setSelectedCategory] = useState('');
  const [selectedDocument, setSelectedDocument] = useState(null);
  const [showModal, setShowModal] = useState(false);
  const [uploadModal, setUploadModal] = useState(false);
  const [previewContent, setPreviewContent] = useState(null);

  // New document form state
  const [newDoc, setNewDoc] = useState({
    patient_id: '',
    clinician_id: '',
    category: 'sick_note',
    title: '',
    description: '',
    content_text: '',
    file_name: '',
    mime_type: '',
  });

  useEffect(() => {
    fetchDocuments();
  }, [selectedCategory]);

  const fetchDocuments = async () => {
    setLoading(true);
    try {
      const params = new URLSearchParams();
      if (selectedCategory) params.append('category', selectedCategory);
      params.append('limit', '20');
      
      const response = await fetch(`${API_BASE_URL}/documents?${params}`);
      if (!response.ok) throw new Error('Failed to fetch documents');
      
      const data = await response.json();
      setDocuments(data.documents || []);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleCategoryChange = (e) => {
    setSelectedCategory(e.target.value);
  };

  const handleViewDocument = async (doc) => {
    setSelectedDocument(doc);
    setShowModal(true);
    
    // Try to fetch document content for preview
    try {
      const response = await fetch(`${API_BASE_URL}/documents/${doc.id}/stream`);
      if (response.ok) {
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/pdf')) {
          const blob = await response.blob();
          const url = URL.createObjectURL(blob);
          setPreviewContent({ type: 'pdf', url });
        } else if (contentType && contentType.includes('image')) {
          const blob = await response.blob();
          const url = URL.createObjectURL(blob);
          setPreviewContent({ type: 'image', url });
        } else if (contentType && contentType.includes('text')) {
          const text = await response.text();
          setPreviewContent({ type: 'text', content: text });
        }
      } else if (doc.content_text) {
        setPreviewContent({ type: 'text', content: doc.content_text });
      }
    } catch (err) {
      console.error('Failed to load document content:', err);
      if (doc.content_text) {
        setPreviewContent({ type: 'text', content: doc.content_text });
      }
    }
  };

  const handleUploadDocument = async () => {
    try {
      const payload = {
        patient_id: newDoc.patient_id,
        clinician_id: newDoc.clinician_id,
        category: newDoc.category,
        title: newDoc.title,
        description: newDoc.description,
        content_text: newDoc.content_text,
        file_name: newDoc.file_name,
        mime_type: newDoc.mime_type,
        is_patient_visible: true,
      };

      const response = await fetch(`${API_BASE_URL}/documents`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });

      if (!response.ok) throw new Error('Failed to upload document');
      
      await response.json();
      setUploadModal(false);
      setNewDoc({
        patient_id: '',
        clinician_id: '',
        category: 'sick_note',
        title: '',
        description: '',
        content_text: '',
        file_name: '',
        mime_type: '',
      });
      fetchDocuments();
    } catch (err) {
      alert('Error uploading document: ' + err.message);
    }
  };

  const getCategoryBadgeVariant = (category) => {
    const variants = {
      sick_note: 'warning',
      referral: 'info',
      prescription: 'success',
      lab_result: 'primary',
      medical_report: 'dark',
      discharge_summary: 'secondary',
      other: 'light',
    };
    return variants[category] || 'secondary';
  };

  const formatCategory = (cat) => {
    return cat.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());
  };

  return (
    <Container>
      <div className="d-flex justify-content-between align-items-center mb-4">
        <h2>Medical Documents</h2>
        <Button variant="primary" onClick={() => setUploadModal(true)}>
          + Upload Document
        </Button>
      </div>

      {error && <Alert variant="danger">{error}</Alert>}

      <Card className="mb-4">
        <Card.Body>
          <Form>
            <Form.Group controlId="categoryFilter">
              <Form.Label>Filter by Category</Form.Label>
              <Form.Select value={selectedCategory} onChange={handleCategoryChange}>
                <option value="">All Categories</option>
                {DOCUMENT_CATEGORIES.map(cat => (
                  <option key={cat} value={cat}>{formatCategory(cat)}</option>
                ))}
              </Form.Select>
            </Form.Group>
          </Form>
        </Card.Body>
      </Card>

      {loading ? (
        <div className="text-center py-5">
          <Spinner animation="border" variant="primary" />
          <p className="mt-2">Loading documents...</p>
        </div>
      ) : documents.length === 0 ? (
        <Alert variant="info">No documents found. Upload a document to get started.</Alert>
      ) : (
        <ListGroup>
          {documents.map(doc => (
            <ListGroup.Item key={doc.id} className="d-flex justify-content-between align-items-center">
              <div>
                <h5 className="mb-1">{doc.title}</h5>
                <div className="text-muted small">
                  <Badge bg={getCategoryBadgeVariant(doc.category)} className="me-2">
                    {formatCategory(doc.category)}
                  </Badge>
                  {doc.file_name && <span>{doc.file_name} • </span>}
                  {doc.file_size_bytes && <span>{(doc.file_size_bytes / 1024).toFixed(1)} KB • </span>}
                  Created: {new Date(doc.created_at).toLocaleDateString()}
                </div>
                {doc.description && <p className="mb-0 mt-1 small">{doc.description}</p>}
              </div>
              <Button variant="outline-primary" size="sm" onClick={() => handleViewDocument(doc)}>
                Preview
              </Button>
            </ListGroup.Item>
          ))}
        </ListGroup>
      )}

      {/* Document Preview Modal */}
      <Modal show={showModal} onHide={() => { setShowModal(false); setPreviewContent(null); }} size="lg">
        <Modal.Header closeButton>
          <Modal.Title>{selectedDocument?.title}</Modal.Title>
        </Modal.Header>
        <Modal.Body style={{ maxHeight: '70vh', overflow: 'auto' }}>
          {selectedDocument && (
            <div>
              <div className="mb-3">
                <Badge bg={getCategoryBadgeVariant(selectedDocument.category)}>
                  {formatCategory(selectedDocument.category)}
                </Badge>
                <span className="ms-2 text-muted">
                  {selectedDocument.file_name || 'No file name'}
                </span>
              </div>

              {previewContent?.type === 'pdf' && (
                <iframe
                  src={previewContent.url}
                  width="100%"
                  height="500px"
                  title="Document Preview"
                  style={{ border: 'none' }}
                />
              )}

              {previewContent?.type === 'image' && (
                <img
                  src={previewContent.url}
                  alt="Document"
                  className="img-fluid"
                />
              )}

              {previewContent?.type === 'text' && (
                <pre style={{ whiteSpace: 'pre-wrap', background: '#f8f9fa', padding: '15px', borderRadius: '5px' }}>
                  {previewContent.content}
                </pre>
              )}

              {!previewContent && (
                <Alert variant="warning">
                  No preview available for this document type.
                </Alert>
              )}

              <div className="mt-3">
                <h6>Document Details</h6>
                <ul className="list-unstyled small">
                  <li><strong>Patient ID:</strong> {selectedDocument.patient_id}</li>
                  <li><strong>Clinician ID:</strong> {selectedDocument.clinician_id}</li>
                  <li><strong>Status:</strong> {selectedDocument.status}</li>
                  <li><strong>Created:</strong> {new Date(selectedDocument.created_at).toLocaleString()}</li>
                  <li><strong>Updated:</strong> {new Date(selectedDocument.updated_at).toLocaleString()}</li>
                </ul>
              </div>
            </div>
          )}
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={() => { setShowModal(false); setPreviewContent(null); }}>
            Close
          </Button>
        </Modal.Footer>
      </Modal>

      {/* Upload Document Modal */}
      <Modal show={uploadModal} onHide={() => setUploadModal(false)}>
        <Modal.Header closeButton>
          <Modal.Title>Upload New Document</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <Form>
            <Form.Group className="mb-3">
              <Form.Label>Patient ID (UUID)</Form.Label>
              <Form.Control
                type="text"
                placeholder="e.g., 123e4567-e89b-12d3-a456-426614174000"
                value={newDoc.patient_id}
                onChange={(e) => setNewDoc({ ...newDoc, patient_id: e.target.value })}
              />
            </Form.Group>
            <Form.Group className="mb-3">
              <Form.Label>Clinician ID (UUID)</Form.Label>
              <Form.Control
                type="text"
                placeholder="e.g., 123e4567-e89b-12d3-a456-426614174000"
                value={newDoc.clinician_id}
                onChange={(e) => setNewDoc({ ...newDoc, clinician_id: e.target.value })}
              />
            </Form.Group>
            <Form.Group className="mb-3">
              <Form.Label>Category</Form.Label>
              <Form.Select
                value={newDoc.category}
                onChange={(e) => setNewDoc({ ...newDoc, category: e.target.value })}
              >
                {DOCUMENT_CATEGORIES.map(cat => (
                  <option key={cat} value={cat}>{formatCategory(cat)}</option>
                ))}
              </Form.Select>
            </Form.Group>
            <Form.Group className="mb-3">
              <Form.Label>Title</Form.Label>
              <Form.Control
                type="text"
                placeholder="Document title"
                value={newDoc.title}
                onChange={(e) => setNewDoc({ ...newDoc, title: e.target.value })}
              />
            </Form.Group>
            <Form.Group className="mb-3">
              <Form.Label>Description</Form.Label>
              <Form.Control
                as="textarea"
                rows={3}
                placeholder="Brief description of the document"
                value={newDoc.description}
                onChange={(e) => setNewDoc({ ...newDoc, description: e.target.value })}
              />
            </Form.Group>
            <Form.Group className="mb-3">
              <Form.Label>Content (Text)</Form.Label>
              <Form.Control
                as="textarea"
                rows={5}
                placeholder="Paste document text content here"
                value={newDoc.content_text}
                onChange={(e) => setNewDoc({ ...newDoc, content_text: e.target.value })}
              />
            </Form.Group>
          </Form>
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={() => setUploadModal(false)}>
            Cancel
          </Button>
          <Button variant="primary" onClick={handleUploadDocument}>
            Upload
          </Button>
        </Modal.Footer>
      </Modal>
    </Container>
  );
}

export default DocumentPreview;
