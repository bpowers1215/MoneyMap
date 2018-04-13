import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {}
};

const mapStateToProps = state => {
	return {}
}

class ConnectedMoneyMapTile extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	render() {
		return (
			 <div className="moneyMapTile tile is-parent is-3">
				<a className="tile is-child box">
					<span class="icon is-large has-text-success">
						<i class="fas fa-4x fa-dollar-sign"></i>
					</span>
					<div className="title is-6">{this.props.name}</div>
				</a>
			</div>
		);
	}
}

const MoneyMapTile = connect(mapStateToProps, mapDispatchToProps)(ConnectedMoneyMapTile);

MoneyMapTile.defaultProps = {
}

MoneyMapTile.propTypes = {
	name: PropTypes.string.isRequired
}

export default MoneyMapTile;
