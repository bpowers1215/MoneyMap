import React, { Component } from 'react';
import { connect } from 'react-redux';
import AccountInfo from './components/AccountInfo';
import './styles.scss';

const mapStateToProps = state => ({
});

class ConnectedAccount extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	render() {
		return (
			<div className="container">
				<h2>Account</h2>
				<AccountInfo />
			</div>
		);
	}
}

const Account = connect(mapStateToProps)(ConnectedAccount);
export default Account;
