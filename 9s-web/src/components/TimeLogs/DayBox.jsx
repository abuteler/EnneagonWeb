import React from 'react';

import Container from '@material-ui/core/Container';
import IconButton from '@material-ui/core/IconButton';
import { Card, CardActions, CardContent } from '@material-ui/core';
import { CalendarToday, Update, ZoomIn } from '@material-ui/icons';

import DayDetailsDialog from './DayDetailsDialog';
import './DayBox.scss';

class DayBox extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      zooming: false,
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
      dayData: { day, month, year, time_entries, },
    } = this.props;
    let minutesWorked = 0;
    time_entries.map((entry)=>{
      let startTime = new Date(`${year} ${day} ${month} ${entry.in}`);
      let endTime = new Date(`${year} ${day} ${month} ${entry.out}`);
      const minutesWorkedEntry = Math.abs(endTime - startTime)/ 1000 /60;
      minutesWorked += minutesWorkedEntry;
      return true;
    });
    return minutesWorked;
  }

  handleClickOpen = () => {
    this.setState(() => ({
      zooming: true,
    }));
  };

  handleClose = () => {
    this.setState(() => ({
      zooming: false,
    }));
  };

  render () {
    const {
      dayData: {
        week_day,
        day,
        month,
        year,
        time_entries,
      },
      notWorked,
    } = this.props;
    const { zooming, totalMinutesWorked } = this.state;
    const minutos = totalMinutesWorked % 60;
    const horas = (totalMinutesWorked - minutos) / 60;

    return (
      <Container className="DayBox_MuiContainer">
        <Card className={notWorked ? 'DayBox_MuiCard disabled' : 'DayBox_MuiCard'}>
          <CardContent className="DayBox_CardContent">
            <div className="DateContainer">
              <CalendarToday fontSize="large" color={notWorked ? 'disabled' : 'primary'} /><span>{month}</span>
              <h4>{day}</h4>
              <h5>{week_day.slice(0,3)}</h5>
            </div>
            <div className="TimeContainer">
              <Update />: {horas}h{minutos > 0 && ` ${minutos}'`}
            </div>

          </CardContent>
          <CardActions>
            <IconButton color="secondary" size="small" disabled={notWorked} onClick={this.handleClickOpen}>
              <ZoomIn />
            </IconButton>
          </CardActions>
        </Card>
        <DayDetailsDialog
          title={`${week_day} ${day} ${month} ${year}`}
          timeEntries={time_entries}
          totalMinutesWorked={totalMinutesWorked}
          handleClose={this.handleClose}
          open={zooming} />
      </Container>
    );
  }
}

export default DayBox;
