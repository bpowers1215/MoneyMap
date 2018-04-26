import { combineReducers } from 'redux'
import data from './data/reducer'
import editMoneyMap from './scenes/EditMoneyMap/reducer'

export default combineReducers({
	data,
	editMoneyMap
})