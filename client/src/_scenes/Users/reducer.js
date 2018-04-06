import { combineReducers } from 'redux'
import account from './Account/reducer'
import data from './data/reducer'

export default combineReducers({
	data,
	account
})