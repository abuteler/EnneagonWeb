import React from 'react';

import Container from '@material-ui/core/Container';
import Card from '@material-ui/core/Card';
import CardActions from '@material-ui/core/CardActions';
import CardContent from '@material-ui/core/CardContent';
import { CalendarToday, AccessTime } from '@material-ui/icons';

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
        // year,
        // time_entries,
      },
      notWorked,
    } = this.props;
    const { totalMinutesWorked } = this.state;

    return (
      <Container className="DayBox_MuiContainer">
        <Card className={ notWorked ? 'DayBox_MuiCard disabled' : 'DayBox_MuiCard'}>
          <CardContent>
            <h4>{week_day.slice(0,2)}</h4>
            <div className="date">
              <CalendarToday /><span>{date}</span>
            </div>
            <div className="time">
              <AccessTime />: {totalMinutesWorked}'
            </div>

          </CardContent>
          <CardActions>
            
          </CardActions>
        </Card>
      </Container>
    );
  }
}

export default DayBox;
