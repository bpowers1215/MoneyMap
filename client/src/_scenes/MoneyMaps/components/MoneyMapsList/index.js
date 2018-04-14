import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import MoneyMapTile from './MoneyMapTile';
import Icon from '~/_components/icon';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {}
};

const mapStateToProps = state => {
	return {}
}

class ConnectedMoneyMapsList extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	componentWillReceiveProps(props) {
		// console.log(props)
	}
	render() {
		return (
			<div className="moneyMapTileList tile is-ancestor">
				{this.props.moneyMaps.map((moneyMap, i) =>
					<MoneyMapTile key={moneyMap.id} name={moneyMap.name} link={`/money_maps/${moneyMap.id}`}>
						<Icon wrapperClassName="is-large has-text-success" className="fas fa-4x fa-dollar-sign"/>
					</MoneyMapTile>
				)}
				<MoneyMapTile key="add-money-map" name="Add" link="/money_maps/add" classModifiers="add-money-map">
					<Icon wrapperClassName="is-large has-text-white" className="fas fa-4x fa-plus"/>
				</MoneyMapTile>
			</div>
		);
	}
}

const MoneyMapsList = connect(mapStateToProps, mapDispatchToProps)(ConnectedMoneyMapsList);

MoneyMapsList.defaultProps = {
}

MoneyMapsList.propTypes = {
	moneyMaps: PropTypes.array.isRequired
}

export default MoneyMapsList;
