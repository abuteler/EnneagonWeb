import React from 'react';

import Container from '@material-ui/core/Container';
import Grid from '@material-ui/core/Grid';
// import { Laptop, Tablet, Smartphone, PresentToAll } from '@material-ui/icons';

import WeekLog from '../components/TimeLogs/WeekLog'
import { p4dLog } from '../constants'
import './VistaTimeLogs.scss';


class VistaTimeLogs extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      totalLogTimeInMinutes: 0,
    } 
  }

  // shouldComponentUpdate(nextProps, nextState) {
  //   console.log(this.state.weekTimeTotalMinutes);
  //   console.log(nextState);
  //   if (this.state.totalLogTimeInMinutes !== nextState.totalLogTimeInMinutes) {
  //     console.log(this.state.weekTimeTotalMinutes);
  //     return true;
  //   } else {
  //     return false;
  //   }
  // }

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
    return (
      <Container id="VistaTimeLogs">
        <h3>Nombre del Proyecto: {p4dLog.projectName}</h3>
        <Grid container direction="row" justify="center" alignItems="center">
          {p4dLog.weeks.map((week, i) => (
            <WeekLog
              key={i}
              weekNumber={i}
              weekData={week}
              handleWeekTimeTotalMinutes={this.handleWeekTimeTotalMinutes.bind(this)}
            />
          ))}
          <Grid item className="TotalTime">{horas} horas, {minutos} minutos</Grid>
        </Grid>
      </Container>
    );
  }
}

export default VistaTimeLogs;