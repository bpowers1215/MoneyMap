import React, { Component } from 'react';
import { connect } from 'react-redux';
import Alerts from '~/_components/alerts';
import './styles.scss';

const mapStateToProps = state => ({
});

class ConnectedMoneyMaps extends Component {
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
								My Money Maps
							</h1>
						</div>
					</div>
				</div>
				<div className="container is-fluid page-content">
					<Alerts />
					My Money Maps Here!
				</div>
			</div>
		);
	}
}

const MoneyMaps = connect(mapStateToProps)(ConnectedMoneyMaps);
export default MoneyMaps;
