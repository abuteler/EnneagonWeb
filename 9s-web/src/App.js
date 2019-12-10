import React from 'react';
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import { ThemeProvider } from '@material-ui/core/styles';
import CssBaseline from '@material-ui/core/CssBaseline';
import Grid from '@material-ui/core/Grid';

import Header from './components/Header/Header';
import MainTheme from './themes/Main.theme';
import VistaLanding from './vistas/Landing.view';
import VistaHome from './vistas/Home.view';
import VistaBlog from './vistas/Blog.view';
import VistaContacto from './vistas/Contacto.view';
import VistaTimeLogs from './vistas/TimeLogs.view';

import './App.scss';

const handleRenderInnerView = (InnerComponent) => {
  // See React HOCs for reference.
  return class extends React.Component {
    // 2do: handle user sessions
    render() {
      return (
        <Grid id="VistaHome" container justify="center">
          <Grid item xs={12}>
            <Header />
          </Grid>
          <Grid item xs={12} className="InnerViewContainer">
            <InnerComponent />
          </Grid>
          <Grid item xs={12}>
            <footer>
              <span>9S Footer</span>
              <a href="http://www.enneagonstudios.com/" target="_blank" rel="noopener noreferrer">
                {/* <img src={footer_logo} alt="Enneagon Studios Logo" /> */} A link
              </a>
            </footer>
          </Grid>
        </Grid>
      );
    };
  };
};

function App() {
  const basePath = '';
  return (
    <ThemeProvider theme={MainTheme}>
      <CssBaseline />
      <div className="App">
        <Grid container justify="center" direction="column" className="full-height">
          <Grid item container className="full-height">
            <section id="App_Vistas">
            <Router>
              <Switch>
                <Route exact path={`${basePath}/`} component={VistaLanding} />
                <Route exact path={`${basePath}/home`} component={handleRenderInnerView(VistaHome)} />
                <Route exact path={`${basePath}/timelog`} component={handleRenderInnerView(VistaTimeLogs)} />
                <Route exact path={`${basePath}/blog`} component={handleRenderInnerView(VistaBlog)} />
                <Route exact path={`${basePath}/contacto`} component={handleRenderInnerView(VistaContacto)} />
                {/* STRIPPED COMPONENTS */}
                <Route exact path={`${basePath}/stripped/timelog`} component={VistaTimeLogs} />
              </Switch>
            </Router>
            </section>
          </Grid>
        </Grid>
      </div>
    </ThemeProvider>
  );
}

export default App;
