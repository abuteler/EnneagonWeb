import { createMuiTheme, responsiveFontSizes } from '@material-ui/core/styles';
import { lightGreen, indigo } from '@material-ui/core/colors';

let EnneaTheme = createMuiTheme({
  palette: {
    primary: lightGreen,
    secondary: indigo,
    background: {
      default: '#000',
    },
  },
  typography: {
    fontSize: 16,
  },
  status: {
    danger: 'orange',
  },
});

EnneaTheme = responsiveFontSizes(EnneaTheme);

export default EnneaTheme;