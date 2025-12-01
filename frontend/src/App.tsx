import { useState } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { Container, Typography, Button, Box, Paper } from '@mui/material';
import Login from './components/Login';
import Signup from './components/Signup';

const queryClient = new QueryClient();

const theme = createTheme({
  palette: {
    mode: 'light',
    primary: {
      main: '#1976d2',
    },
    secondary: {
      main: '#dc004e',
    },
  },
});

interface User {
  id: string;
  email: string;
  name: string;
}

function App() {
  const [user, setUser] = useState<User | null>(null);
  const [mode, setMode] = useState<'login' | 'signup'>('login');

  const handleLogout = async () => {
    await fetch('/api/auth/logout', {
      method: 'POST',
      credentials: 'include',
    });
    setUser(null);
  };

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <Container>
          <Box sx={{ my: 4 }}>
            <Typography
              variant="h2"
              component="h1"
              gutterBottom
              sx={{ textAlign: 'center' }}
            >
              Enoki
            </Typography>

            {user ? (
              <Paper elevation={3} sx={{ p: 4, maxWidth: 400, mx: 'auto' }}>
                <Typography variant="h5" gutterBottom>
                  Welcome, {user.name}!
                </Typography>
                <Typography variant="body1" color="text.secondary" gutterBottom>
                  Email: {user.email}
                </Typography>
                <Button
                  fullWidth
                  variant="outlined"
                  onClick={handleLogout}
                  sx={{ mt: 2 }}
                >
                  Logout
                </Button>
              </Paper>
            ) : (
              <Box sx={{ mt: 4 }}>
                {mode === 'login' ? (
                  <Login
                    onSuccess={setUser}
                    onToggleMode={() => setMode('signup')}
                  />
                ) : (
                  <Signup
                    onSuccess={setUser}
                    onToggleMode={() => setMode('login')}
                  />
                )}
              </Box>
            )}
          </Box>
        </Container>
        <ReactQueryDevtools initialIsOpen={false} />
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
