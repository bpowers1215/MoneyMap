import { alertConstants } from '~/_constants';

const initialState = {
	alerts: []
}

const alertReducer = (state = initialState, action) => {
	
	switch (action.type) {
		case alertConstants.ADD_ALERT: {
			return {
				...state,
				alerts: [...state.alerts, action.alert]
			}
		}
		case alertConstants.CLEAR_ALERTS: {
			return {
				...state,
				alerts: []
			}
		}
		case alertConstants.REMOVE_ALERT: {
			return {
				...state,
				alerts: [
					...state.alerts.slice(0, action.id),
					...state.alerts.slice(action.id+1)
				]
			}
		}
		default:
			return state;
	}
};

export default alertReducer;