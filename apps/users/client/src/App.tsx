// import logo from './logo.svg';
import styles from './App.module.css';
import { Route, Routes, Link } from 'solid-app-router';
import { lazy } from 'solid-js';
import Button from '@suid/material/Button';
import { purple } from '@suid/material/colors';
import { createTheme, ThemeProvider } from '@suid/material/styles';

const Predictions = lazy(() => import('./components/Predictions/Predictions'));

const theme = createTheme({
  palette: {
    primary: {
      // Purple and green play nicely together.
      main: purple[300],
    },
    secondary: {
      // This is green.A700 as hex.
      main: '#cb11c8',
    },
  },
});

// function getUsers() {
//   return axios.get('http://localhost:8080/api/users').then((r) => r.data);
// }

// const Predictions = () => {
//   createEffect(() => {
//     getUsers().then((res) => {
//       console.log('res!', res);
//     });
//   });
//   return <div>Predictions</div>;
// };
console.log('BASE_URL', import.meta.env.BASE_URL);
const About = () => {
  return <div>About</div>;
};

function App() {
  return (
    <ThemeProvider theme={theme}>
      <div class={styles.App}>
        <h1 class="underline font-bold">Welcome users-client</h1>
        <Button>mui button</Button>
        <nav>
          <Link href="/about">About</Link>
          <Link href="/">Home</Link>
        </nav>
        <Routes>
          <Route path="/" element={<Predictions />} />
          <Route path="/about" element={<About />} />
          {/*<Route path="/*all" element={<NotFound />} />*/}
        </Routes>
      </div>
    </ThemeProvider>
  );
}

export default App;
