import { CssBaseline, ThemeProvider } from "@material-ui/core";
import { Mapping } from "./components/Mapping";
import theme from "./theme";

function App() {
  return (
    <ThemeProvider theme={theme} >

      <CssBaseline />
      <Mapping />

    </ThemeProvider>
  );
}

export default App;
