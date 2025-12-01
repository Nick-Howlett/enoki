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

interface LoginData {
  email: string;
  password: string;
}

interface AuthResponse {
  user: {
    id: string;
    email: string;
    name: string;
  };
}

interface LoginProps {
  onSuccess: (user: AuthResponse['user']) => void;
  onToggleMode: () => void;
}

async function loginUser(data: LoginData): Promise<AuthResponse> {
  const response = await fetch('/api/auth/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
    credentials: 'include',
  });

  if (!response.ok) {
    throw new Error('Invalid credentials');
  }

  return response.json();
}

export default function Login({ onSuccess, onToggleMode }: LoginProps) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const mutation = useMutation({
    mutationFn: loginUser,
    onSuccess: (data) => {
      onSuccess(data.user);
    },
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    mutation.mutate({ email, password });
  };

  return (
    <Paper elevation={3} sx={{ p: 4, maxWidth: 400, mx: 'auto' }}>
      <Typography variant="h5" component="h2" gutterBottom>
        Login
      </Typography>

      {mutation.isError && (
        <Alert severity="error" sx={{ mb: 2 }}>
          {mutation.error.message}
        </Alert>
      )}

      <Box component="form" onSubmit={handleSubmit} sx={{ mt: 2 }}>
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
          {mutation.isPending ? 'Logging in...' : 'Login'}
        </Button>

        <Button
          fullWidth
          variant="text"
          sx={{ mt: 1 }}
          onClick={onToggleMode}
        >
          Need an account? Sign up
        </Button>
      </Box>
    </Paper>
  );
}
