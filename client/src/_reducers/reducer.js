import { combineReducers } from 'redux'
import dataReducer from '~/_data/reducer';
import scenesReducer from '~/_scenes/reducer';
import alertsReducer from './alerts.reducer';

export default combineReducers({
	data: dataReducer,
	scenes: scenesReducer,
	alerts: alertsReducer
})