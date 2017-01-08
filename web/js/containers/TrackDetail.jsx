import React from 'react';
import {
  Card,
  CardActions,
  CardHeader,
  CardMedia,
  CardTitle,
  CardText,
} from 'material-ui/Card';
import { List, ListItem }          from 'material-ui/List';
import RaisedButton                from 'material-ui/RaisedButton';
import { connect }                 from 'react-redux';
import { Status }                  from '../reducers/track';
import { fetchTrack, updateTrack } from '../actions';
import tryGet                      from '../utils/tryGet';
import datePrettify                from '../utils/datePrettify';

const NO_IMAGE        = '/web/no_image.png';
const DEAD_IMAGE      = '/web/dead_image.png';
const images = {
  YouTube:    '/web/youtube.png',
  SoundCloud: '/web/soundcloud.png',
};

class TrackDetail extends React.Component {
  static get propTypes() {
    return {
      item:                    React.PropTypes.object.isRequired,
      status:                  React.PropTypes.string.isRequired,
      fetchTrackIfNeeded:      React.PropTypes.func,
      handleUpdateButtonClick: React.PropTypes.func,
    };
  }
  componentDidMount() {
    this.props.fetchTrackIfNeeded(this.props.status);
  }
  componentDidUpdate() {
    this.props.fetchTrackIfNeeded(this.props.status);
  }
  render() {
    const id          = tryGet(this.props.item, 'id', 'unknown id');
    const state       = tryGet(this.props.item, 'state', 'unknown state');
    const title       = tryGet(this.props.item, 'title', 'No Title');
    const description = tryGet(this.props.item, 'description', 'No Description');
    const provider    = tryGet(this.props.item, 'provider', 'No Service');
    const artworkUrl  = tryGet(this.props.item, 'artwork_url', NO_IMAGE);
    const publishedAt = datePrettify(tryGet(this.props.item, 'published_at', null));
    const createdAt   = datePrettify(tryGet(this.props.item, 'created_at', null));
    const updatedAt   = datePrettify(tryGet(this.props.item, 'updated_at', null));
    const overlay = (
      <CardTitle
        title={title}
        subtitle={description}
      />
    );
    const style = {
      margin: 'auto',
      width:  'calc(75vh)',
    };
    return (
      <Card>
        <CardHeader
          title={tryGet(this.props.item, 'artist', 'No Artist')}
          subtitle={provider}
          avatar={images[provider]}
        />
        <CardMedia style={style} overlay={overlay} >
          <img role="presentation" src={state === 'alive' ? artworkUrl : DEAD_IMAGE} />
        </CardMedia>
        <CardTitle title={title} />
        <CardActions>
          <RaisedButton
            primary
            label={`View on ${provider}`}
            href={this.props.item.url}
          />
          <RaisedButton
            primary
            label="Update"
            onClick={() => this.props.handleUpdateButtonClick(this.props.item)}
          />
        </CardActions>
        <CardText>
          <List>
            <ListItem primaryText="id" secondaryText={id} />
            <ListItem primaryText="state" secondaryText={state} />
            <ListItem primaryText="published" secondaryText={publishedAt} />
            <ListItem primaryText="created" secondaryText={createdAt} />
            <ListItem primaryText="updated" secondaryText={updatedAt} />
          </List>
        </CardText>
      </Card>
    );
  }
}

function mapStateToProps(state) {
  return {
    ...state.track,
  };
}

function mapDispatchToProps(dispatch, ownProps) {
  const trackId = ownProps.params.track_id;
  return {
    fetchTrackIfNeeded: (status) => {
      if (status === Status.Dirty) {
        dispatch(fetchTrack(trackId));
      }
    },
    handleUpdateButtonClick: track => dispatch(updateTrack(track.id)),
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(TrackDetail);
