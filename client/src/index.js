import React from 'react';
import ReactDOM from 'react-dom';
import { Provider } from 'react-redux';
import store from "./_helpers/store";
import App from './App';
import registerServiceWorker from './registerServiceWorker';
import './index.css';
import 'bootstrap/dist/css/bootstrap.css';

ReactDOM.render(
	<Provider store={store}>
		<App />
	</Provider>
	, document.getElementById('root')
);
registerServiceWorker();
