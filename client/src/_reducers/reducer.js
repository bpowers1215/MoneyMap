import { combineReducers } from 'redux'
import data from '~/_data/reducer';
import scenes from '~/_scenes/reducer';

export default combineReducers({
	data,
	scenes
})