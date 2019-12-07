import React from 'react';

import Container from '@material-ui/core/Container';
import IconButton from '@material-ui/core/IconButton';
import { Card, CardActions, CardContent } from '@material-ui/core';
import { CalendarToday, Timelapse, ZoomIn } from '@material-ui/icons';

import './DayBox.scss';

class DayBox extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      totalMinutesWorked: 0,
    } 
  }

  componentDidMount() {
    const { handleDayWorkedTimeSum } = this.props;
    const totalMinutesWorked = this.calculateTotalTimeWorked();
    if (totalMinutesWorked > 0) {
      this.setState(() => ({
        totalMinutesWorked,
      }));
      handleDayWorkedTimeSum(totalMinutesWorked);
    }
  }

  calculateTotalTimeWorked = () => {
    const {
      dayData: { date, year, time_entries, },
    } = this.props;
    let minutesWorked = 0;
    time_entries.map((entry)=>{
      let startTime = new Date(`${year} ${date} ${entry.in}`);
      let endTime = new Date(`${year} ${date} ${entry.out}`);
      const minutesWorkedEntry = Math.abs(endTime - startTime)/ 1000 /60;
      minutesWorked += minutesWorkedEntry;
      return true;
    });
    return minutesWorked;
  }

  render () {
    const {
      dayData: {
        week_day,
        date,
      },
      notWorked,
    } = this.props;
    const { totalMinutesWorked } = this.state;
    const minutos = totalMinutesWorked % 60;
    const horas = (totalMinutesWorked - minutos) / 60;

    return (
      <Container className="DayBox_MuiContainer">
        <Card className={notWorked ? 'DayBox_MuiCard disabled' : 'DayBox_MuiCard'}>
          <CardContent className="DayBox_CardContent">
            <h4>{week_day.slice(0,2)}</h4>
            <div className="date">
              <CalendarToday fontSize="large" color={notWorked ? 'disabled' : 'primary'} /><span>{date}</span>
            </div>
            <div className="time">
              <Timelapse />: {horas}h{minutos > 0 && ` ${minutos}'`}
            </div>

          </CardContent>
          <CardActions>
            <IconButton color="secondary" size="small" disabled={notWorked}>
              <ZoomIn />
            </IconButton>
          </CardActions>
        </Card>
      </Container>
    );
  }
}

export default DayBox;
