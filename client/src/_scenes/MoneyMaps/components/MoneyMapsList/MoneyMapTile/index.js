import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import AppLink from '~/_components/appLink';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {}
};

const mapStateToProps = state => {
	return {}
}

class ConnectedMoneyMapTile extends Component {
	render() {
		return (
			 <div className="moneyMapTile tile is-parent is-3">
			 	<AppLink className={"tile is-child box "+this.props.classModifiers} to={this.props.link}>
					{this.props.children}
					<div className="title is-6">{this.props.name}</div>
				</AppLink>
			</div>
		);
	}
}

const MoneyMapTile = connect(mapStateToProps, mapDispatchToProps)(ConnectedMoneyMapTile);

MoneyMapTile.defaultProps = {
	classModifiers: ''
}

MoneyMapTile.propTypes = {
	name: PropTypes.string.isRequired,
	link: PropTypes.string.isRequired,
	classModifiers: PropTypes.string
}

export default MoneyMapTile;
