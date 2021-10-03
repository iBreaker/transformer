import React from 'react';
import Button from '@mui/material/Button';

export type HomeBlockProps = {
    url: string,
    title: string,
    description: string,
};

const HomeBlock = (props: HomeBlockProps) => (
    <a target="_blank" rel="noreferrer" className='home-block-anchor' href={props.url}>
        <div className='home-block'>
            <h4 className='home-block-title'>📚 {props.title}</h4>
            <p>{props.description}</p>
            <Button variant="contained">Learn More</Button>
        </div>
    </a>
)

export default HomeBlock;
