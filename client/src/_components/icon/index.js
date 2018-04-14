import React, { Component } from 'react';
import PropTypes from 'prop-types';

class Icon extends Component {
	render() {
		return (
			<span className={"icon "+this.props.wrapperClassName}>
				<i className={this.props.className}></i>
			</span>
		);
	}
}

Icon.defaultProps = {
	wrapperClassName: '',
	className: 'fas'
}

Icon.propTypes = {
	wrapperClassName: PropTypes.string,
	className: PropTypes.string
}

export default Icon;
