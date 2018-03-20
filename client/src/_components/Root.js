import React from 'react';
import PropTypes from 'prop-types';
import { Provider } from 'react-redux';
import { Router } from 'react-router-dom';
import { history } from '~/_helpers/history';
import App from './App';

const Root = ({store}) => (
	<Provider store={store}>
		<Router history={history}>
			<App />
		</Router>
	</Provider>
)

Root.propTypes = {
	store: PropTypes.object.isRequired
}

export default Root;