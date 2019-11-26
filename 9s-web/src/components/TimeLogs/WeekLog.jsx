import React from 'react';

import Grid from '@material-ui/core/Grid';

import DayBox from './DayBox'
import './DayBox.scss';

class WeekLog extends React.PureComponent {
  constructor(props) {
    super(props);
    this.state = {
      weekTimeTotalMinutes: 0,
      handled: false,
    } 
  }

  componentDidUpdate(prevProps) {
    const { handleWeekTimeTotalMinutes } = this.props;
    const { weekTimeTotalMinutes, handled } = this.state;
    // console.log('prevProps', prevProps);
    // console.log('state', this.state);
    if (weekTimeTotalMinutes > 0 && handled === false) {
      handleWeekTimeTotalMinutes(weekTimeTotalMinutes);
      this.setState(() => ({
        handled: true,
      }));
    }
  }

  handleDayWorkedTimeSum = (totalMinutesWorked) => {
    this.setState((state) => ({
      weekTimeTotalMinutes: state.weekTimeTotalMinutes + totalMinutesWorked,
    }));
  }

  renderDay = (day) => (
    day.date !== 'dd MMM' &&
      <DayBox
        dayData={day}
        notWorked={day.time_entries.length === 0}
        handleDayWorkedTimeSum={this.handleDayWorkedTimeSum.bind(this)}
      />
  );

  render () {
    const { weekData } = this.props;

    return (
      <Grid item container className="Week_MuiGrid_item_container" direction="row">
        {weekData.map((day, i) => (
          <div key={i} className="DayBox_container">
            {this.renderDay(day)}
          </div>
        ))}
      </Grid>
    );
  }
}

export default WeekLog;
