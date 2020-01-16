import React from 'react';

import { ThemeProvider } from '@material-ui/core/styles';
import Container from '@material-ui/core/Container';
import Typography from '@material-ui/core/Typography';
import Grid from '@material-ui/core/Grid';

import WeekLog from '../components/TimeLogs/WeekLog'
import TimelogsTheme from '../themes/Timelogs.theme';
import './TimeLogs.view.scss';

import { p4dLog as NovLog } from '../assets/json/p4d_batch.2'
import { p4dLog as DecLog } from '../assets/json/p4d_batch.3'

class VistaTimeLogs extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      totalLogTimeInMinutes: 0,
    } 
  }

  handleWeekTimeTotalMinutes = (weekTimeTotalMinutes) => {
    if (weekTimeTotalMinutes !== this.state.totalLogTimeInMinutes) {
      this.setState((state) => ({
        totalLogTimeInMinutes: state.totalLogTimeInMinutes + weekTimeTotalMinutes,
      }));
    }
  }

  render () {
    const { totalLogTimeInMinutes } = this.state;
    const minutos = totalLogTimeInMinutes % 60;
    const horas = (totalLogTimeInMinutes - minutos) / 60;

    const p4dLog = (this.props.location.search === "?nov") ? NovLog : DecLog;

    return (
      <ThemeProvider theme={TimelogsTheme}>
        <Container id="VistaTimeLogs">
          <Typography variant="h3">
            Nombre del Proyecto: {p4dLog.projectName}
          </Typography>
          <Grid container direction="row" justify="center" alignItems="center">
            {p4dLog.weeks.map((week, i) => (
              <WeekLog
                key={i}
                weekNumber={i}
                weekData={week}
                handleWeekTimeTotalMinutes={this.handleWeekTimeTotalMinutes.bind(this)}
              />
            ))}
            <Grid item className="TotalTime">
              <Typography variant="h4">{horas} horas, {minutos} minutos</Typography>
            </Grid>
          </Grid>
        </Container>
      </ThemeProvider>
    );
  }
}

export default VistaTimeLogs;
