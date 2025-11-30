import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { Container, Typography, Button, Box } from '@mui/material';
import { useState } from 'react';

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

function App() {
  const [count, setCount] = useState(0);

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <Container>
          <Box sx={{ my: 4, textAlign: 'center' }}>
            <Typography variant="h2" component="h1" gutterBottom>
              Enoki
            </Typography>
            <Typography variant="h5" component="h2" gutterBottom>
              Vite + React + Material UI + Tanstack Query
            </Typography>
            <Box sx={{ mt: 4 }}>
              <Button
                variant="contained"
                onClick={() => setCount((count) => count + 1)}
              >
                Count is {count}
              </Button>
            </Box>
            <Typography variant="body2" sx={{ mt: 4 }} color="text.secondary">
              Edit src/App.tsx and save to test HMR
            </Typography>
          </Box>
        </Container>
        <ReactQueryDevtools initialIsOpen={false} />
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
