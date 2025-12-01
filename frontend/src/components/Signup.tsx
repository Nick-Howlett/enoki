import { useState } from 'react';
import { useMutation } from '@tanstack/react-query';
import {
  Box,
  Button,
  TextField,
  Typography,
  Paper,
  Alert,
} from '@mui/material';

interface SignupData {
  email: string;
  name: string;
  password: string;
}

interface AuthResponse {
  user: {
    id: string;
    email: string;
    name: string;
  };
}

interface SignupProps {
  onSuccess: (user: AuthResponse['user']) => void;
  onToggleMode: () => void;
}

async function signupUser(data: SignupData): Promise<AuthResponse> {
  const response = await fetch('/api/auth/signup', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
    credentials: 'include',
  });

  if (!response.ok) {
    throw new Error('Failed to create account');
  }

  return response.json();
}

export default function Signup({ onSuccess, onToggleMode }: SignupProps) {
  const [email, setEmail] = useState('');
  const [name, setName] = useState('');
  const [password, setPassword] = useState('');

  const mutation = useMutation({
    mutationFn: signupUser,
    onSuccess: (data) => {
      onSuccess(data.user);
    },
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    mutation.mutate({ email, name, password });
  };

  return (
    <Paper elevation={3} sx={{ p: 4, maxWidth: 400, mx: 'auto' }}>
      <Typography variant="h5" component="h2" gutterBottom>
        Sign Up
      </Typography>

      {mutation.isError && (
        <Alert severity="error" sx={{ mb: 2 }}>
          {mutation.error.message}
        </Alert>
      )}

      <Box component="form" onSubmit={handleSubmit} sx={{ mt: 2 }}>
        <TextField
          fullWidth
          label="Name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          margin="normal"
          required
        />

        <TextField
          fullWidth
          label="Email"
          type="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          margin="normal"
          required
        />

        <TextField
          fullWidth
          label="Password"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          margin="normal"
          required
        />

        <Button
          type="submit"
          fullWidth
          variant="contained"
          sx={{ mt: 3 }}
          disabled={mutation.isPending}
        >
          {mutation.isPending ? 'Creating account...' : 'Sign Up'}
        </Button>

        <Button
          fullWidth
          variant="text"
          sx={{ mt: 1 }}
          onClick={onToggleMode}
        >
          Already have an account? Login
        </Button>
      </Box>
    </Paper>
  );
}
