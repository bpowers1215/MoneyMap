import React, { Component } from 'react';
import PropTypes from 'prop-types';

/*
* Field Component
*/
class StaticField extends Component {
	constructor(props) {
		super(props);
		this.state = {}
	}
	render() {
		return (
			<div className={this.props.fieldClasses}>
				<label className="label label-static">{this.props.label}</label>
				<div className={this.props.controlClasses} onClick={this.props.onClickHandler}>
					<span>{this.props.value}</span>
					{this.props.children}
				</div>
			</div>
		)
	}
}

StaticField.defaultProps = {
	fieldClasses: 'field static-field',
	controlClasses: 'control'
}

StaticField.propTypes = {
	label: PropTypes.string,
	value: PropTypes.string,
	fieldClasses: PropTypes.string,
	controlClasses: PropTypes.string,
	onClickHandler: PropTypes.func
}

export { StaticField };