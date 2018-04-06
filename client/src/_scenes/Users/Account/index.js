import React, { Component } from 'react';
import { connect } from 'react-redux';
import Alerts from '~/_components/alerts';
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
			<div>
				<div className="hero is-primary">
					<div className="hero-body">
						<div className="container">
							<h1 className="title">
								Account
							</h1>
						</div>
					</div>
				</div>
				<div className="container is-fluid page-content">
					<Alerts />
					<AccountInfo />
				</div>
			</div>
		);
	}
}

const Account = connect(mapStateToProps)(ConnectedAccount);
export default Account;
