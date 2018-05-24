import { alertConstants } from '~/_constants';
import { history } from '~/_helpers/history';

/*
* pushToHistory
*
* Directs the user to path
* If specified,clears alerts prior to navigation
*/
const navigate = (path, clearAlerts) => {
	return dispatch => {
		if (clearAlerts)
			dispatch(clearAlerts());
		history.push(path);

		function clearAlerts(){
			return { 
				type: alertConstants.CLEAR_ALERTS
			}
		};
	}
}



const appLinkActions = {
	navigate
}

export default appLinkActions;